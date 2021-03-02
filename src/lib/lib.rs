use futures::StreamExt;
use soup::{NodeExt, QueryBuilderExt};

#[derive(Debug)]
pub struct Houseplant {
    pub name: String,
    pub attributes: Attributes,
}
pub type Attributes = Vec<Attribute>;
#[derive(Debug)]
pub struct Attribute {
    pub parameter: String,
    pub value: String,
}

pub struct Scraper {
    client: reqwest::Client,
    concurrent_tasks: usize,
}

impl Scraper {
    pub fn new(concurrent_tasks: usize) -> Self {
        Scraper {
            client: reqwest::Client::new(),
            concurrent_tasks,
        }
    }

    pub async fn scraper(&self) -> Result<Vec<Houseplant>, reqwest::Error> {
        // Get title page
        let url = "https://komnatnie-rastenija.ru/";
        let response = self.client.get(url).send().await?;
        let html = response.text().await?;
        // Parse categories ('Рубрики')
        let soup = soup::Soup::new(&html);
        let urls = soup
            .class("cat-item")
            .find_all()
            .filter_map(|node| node.children().next())
            .filter_map(|node| node.get("href"))
            .collect::<Vec<String>>();

        // For each category get all plants urls
        let mut plants_url = futures::stream::iter(urls)
            .map(|url| async move { self.parse_category(&url).await })
            .buffer_unordered(self.concurrent_tasks)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .flatten()
            .collect::<Vec<String>>();

        // Remove duplicates
        plants_url.sort_unstable();
        plants_url.dedup();

        // Parse all plants info
        let plants_info = futures::stream::iter(plants_url)
            .map(|url| async move { self.parse_houseplant(&url).await })
            .buffer_unordered(self.concurrent_tasks)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<Houseplant>>();

        Ok(plants_info)
    }

    fn page_count(&self, html: &str) -> usize {
        let soup = soup::Soup::new(&html);
        if let Some(node) = soup.attr("class", "nav-links").find() {
            let count = node.children().count();
            node.children()
                .nth(count - 3)
                .unwrap()
                .text()
                .parse::<usize>()
                .unwrap()
        } else {
            1
        }
    }

    async fn parse_titles(&self, url: &str) -> Option<Vec<String>> {
        let response = self.client.get(url).send().await.ok()?;
        let html = response.text().await.ok()?;
        let soup = soup::Soup::new(&html);
        let url_list = soup
            .tag("a")
            .attr("itemprop", "url")
            .find_all()
            .map(|a| a.get("href").unwrap())
            .collect::<Vec<String>>();
        Some(url_list)
    }

    async fn parse_category(&self, url: &str) -> Option<Vec<String>> {
        // Get page count
        let response = self.client.get(url).send().await.ok()?;
        let html = response.text().await.ok()?;
        // Get page count
        let page_count = self.page_count(&html);
        // Create urls for all pages
        let pages = (1..=page_count)
            .map(|page| url.to_owned() + "/page/" + page.to_string().as_str())
            .collect::<Vec<String>>();
        // Parse plants urls
        let plants_url = futures::stream::iter(pages)
            .map(|url| async move { self.parse_titles(&url).await })
            .buffer_unordered(self.concurrent_tasks)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .flatten()
            .collect::<Vec<String>>();
        Some(plants_url)
    }

    async fn parse_houseplant(&self, url: &str) -> Option<Houseplant> {
        let response = self.client.get(url).send().await.ok()?;
        let html = response.text().await.ok()?;

        let soup = soup::Soup::new(&html);
        // Parse plant name
        let plant_name = soup
            .attr("class", "entry-title")
            .find()
            .expect("Can't find title")
            .text();
        let plant_name = plant_name.split('—').next().unwrap().to_string();

        // Parse table
        if let Some(node) = soup
            .tag("td")
            .find_all()
            .find(|node| node.text().to_lowercase().contains("полив"))
        {
            // Parse table's rows
            let body = node.parent().unwrap().parent().unwrap();
            let nodes = body.children().filter(|node| node.name() == "tr");
            let attributes = nodes
                .map(|tr| {
                    let mut children = tr.children();
                    let td1 = children.next().expect("can't find td").text();
                    let td2 = children.next().expect("can't find td").text();
                    Attribute {
                        parameter: td1,
                        value: td2,
                    }
                })
                .collect::<Attributes>();
            Some(Houseplant {
                name: plant_name,
                attributes,
            })
        } else {
            None
        }
    }
}

impl Default for Scraper {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            concurrent_tasks: 5,
        }
    }
}
