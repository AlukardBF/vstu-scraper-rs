pub mod database;

use anyhow::Result;
use futures::StreamExt;
use soup::{NodeExt, QueryBuilderExt};

#[derive(Debug)]
pub struct Houseplant {
    pub name: String,
    pub attributes: Attributes,
}
#[derive(Debug, Default)]
pub struct Attributes {
    pub temperature: Option<Attribute>,
    pub humidity: Option<Attribute>,
    pub illumination: Option<Attribute>,
    pub watering: Option<Attribute>,
    pub soil: Option<Attribute>,
    pub fertilizer: Option<Attribute>,
    pub transplant: Option<Attribute>,
    pub propagation: Option<Attribute>,
    pub features: Option<Attribute>,
}
#[derive(Debug)]
pub struct Attribute {
    pub parameter: String,
    pub value: String,
}

pub struct Scraper<T: database::Database> {
    client: reqwest::Client,
    concurrent_tasks: usize,
    database: Option<T>,
}

impl<T> Scraper<T>
where
    T: database::Database,
{
    pub fn new(concurrent_tasks: usize, database: Option<T>) -> Self {
        Scraper {
            client: reqwest::Client::new(),
            concurrent_tasks,
            database,
        }
    }

    pub async fn scraper(&self) -> Result<Vec<Houseplant>> {
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
            .map(|url| async move {
                let opt_plant = self.parse_houseplant(&url).await;
                let plant = opt_plant.as_ref().unwrap();
                if let Some(db) = &self.database {
                    db.insert(plant)
                        .await
                        .expect("Failed to insert info into database");
                }
                opt_plant
            })
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
            let list = nodes
                .map(|tr| {
                    let mut children = tr.children();
                    let td1 = children.next().expect("can't find td").text();
                    let td2 = children.next().expect("can't find td").text();
                    Attribute {
                        parameter: td1,
                        value: td2,
                    }
                })
                .collect::<Vec<Attribute>>();
            let attrs = self.parse_attributes(list).ok()?;
            Some(Houseplant {
                name: plant_name,
                attributes: attrs,
            })
        } else {
            None
        }
    }

    fn parse_attributes(&self, list: Vec<Attribute>) -> Result<Attributes> {
        lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(
                concat!(
                    r#"(?P<temp>температ)|"#,
                    r#"(?P<hum>влажн)|"#,
                    r#"(?P<illum>освещен)|"#,
                    r#"(?P<water>полив)|"#,
                    r#"(?P<soil>грунт)|"#,
                    r#"(?P<fertil>подкорм|удобрен)|"#,
                    r#"(?P<trans>пересад)|"#,
                    r#"(?P<prop>размнож)|"#,
                    r#"(?P<feature>особен)"#
                )
            ).unwrap();
        }

        let mut attrs = Attributes::default();
        for item in list {
            let param = item.parameter.to_lowercase();
            let caps: Option<regex::Captures> = RE.captures(&param);
            if caps.is_none() {
                attrs.features = Some(item);
                continue;
            }
            let caps = caps.unwrap();
            if let Some(_) = caps.name("temp") {
                attrs.temperature = Some(item);
            } else if let Some(_) = caps.name("hum") {
                attrs.humidity = Some(item);
            } else if let Some(_) = caps.name("illum") {
                attrs.illumination = Some(item);
            } else if let Some(_) = caps.name("water") {
                attrs.watering = Some(item);
            } else if let Some(_) = caps.name("soil") {
                attrs.soil = Some(item);
            } else if let Some(_) = caps.name("fertil") {
                attrs.fertilizer = Some(item);
            } else if let Some(_) = caps.name("trans") {
                attrs.transplant = Some(item);
            } else if let Some(_) = caps.name("prop") {
                attrs.propagation = Some(item);
            } else if let Some(_) = caps.name("feature") {
                attrs.features = Some(item);
            }
        }
        Ok(attrs)
    }
}

impl<T> Default for Scraper<T>
where
    T: database::Database,
{
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            concurrent_tasks: 5,
            database: None,
        }
    }
}

trait OptArg {
    fn get_value(&self) -> Option<&str>;
}

impl OptArg for Option<Attribute> {
    fn get_value(&self) -> Option<&str> {
        self.as_ref()
            .map(|x| Some(x.value.as_str()))
            .unwrap_or(None)
    }
}
