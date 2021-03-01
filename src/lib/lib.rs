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

use soup::{NodeExt, QueryBuilderExt};

pub async fn scraper() -> Result<Vec<Houseplant>, reqwest::Error> {
    let plants_url: Vec<&str> = vec![
        "https://komnatnie-rastenija.ru/abutilon-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/adenium-uhod-v-domashnih-uslovijah-peresadka-foto-vidov/",
        "https://komnatnie-rastenija.ru/adiantum-posadka-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/adiantum-venerin-volos-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/afelandra-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/agava-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/aglaonema-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/ahimenes-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/akalifa-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/allamanda-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/aloje-stoletnik-vyrashhivanie-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/alokazija-domashnjaja-vyrashhivanie-i-uhod/",
        "https://komnatnie-rastenija.ru/amarillis-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/amorfofallus-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/aptenija-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/araukarija-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/ardizija-uhod-v-domashnih-uslovijah-razmnozhenie-foto-vidov/",
        "https://komnatnie-rastenija.ru/asparagus-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/aspidistra-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/asplenium-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/aukuba-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/azalija-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/balzamin-uollera-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-sortov/",
        "https://komnatnie-rastenija.ru/banan-domashnij-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/begonija-klubnevaja-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/beloperone-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/beresklet-japonskij-komnatnyj-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/bilbergija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/bokarneja-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/brovallija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/brugmansija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/bugenvillija-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/ceropegija-uhod-v-domashnih-uslovijah-foto-vidov-razmnozhenie/",
        "https://komnatnie-rastenija.ru/cikas-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov-rastenija/",
        "https://komnatnie-rastenija.ru/ciklamen-domashnij-uslovija-uhoda-foto/",
        "https://komnatnie-rastenija.ru/cimbidium-uhod-v-domashnih-uslovijah-foto-vidov-peresadka-i-razmnozhenie/",
        "https://komnatnie-rastenija.ru/cinerarija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-sortov/",
        "https://komnatnie-rastenija.ru/ciperus-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/cissus-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/cvetok-oleandr-v-domashnih-uslovijah-uhod-i-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/davallija-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/diffenbahija-v-domashnih-uslovijah-uhod-i-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/dipladenija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-razmnozhenie-foto-vidov/",
        "https://komnatnie-rastenija.ru/dizigoteka-posadka-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/dracena-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/duranta-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/fatshedera-lize-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/fatsija-japonskaja-vyrashhivanie-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/fialka-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/fikus-bendzhamina-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/fikus-bengalskij-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/fikus-kauchukonosnyj-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/fikus-lirovidnyj-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/fikus-mikrokarpa-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-rastenija/",
        "https://komnatnie-rastenija.ru/fikus-svjashhennyj-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/filodendron-uhod-v-domashnih-uslovijah-vidy-s-foto-i-nazvanijami/",
        "https://komnatnie-rastenija.ru/fittonija-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/frezija-posadka-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/fuksija-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/gardenija-zhasminovidnaja-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/gasterija-uhod-v-domashnih-uslovijah-foto-vidov-razmnozhenie/",
        "https://komnatnie-rastenija.ru/gelikonija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/gemantus-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/geran-domashnjaja-vyrashhivanie-i-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/gerbera-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/giacint-uhod-v-domashnih-uslovijah-v-gorshke-foto-sortov-i-vidov/",
        "https://komnatnie-rastenija.ru/gibiskus-kitajskij-posadka-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/gimenokallis-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/gimnokalicium-posadka-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/gippeastrum-uhod-v-domashnih-uslovijah-foto-sortov/",
        "https://komnatnie-rastenija.ru/gloksinija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/glorioza-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/gortenzija-vyrashhivanie-uhod-v-domashnih-uslovijah-foto-sortov/",
        "https://komnatnie-rastenija.ru/granat-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/gujernija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/guzmanija-uhod-v-domashnih-uslovijah-foto-vidov-peresadka-i-razmnozhenie/",
        "https://komnatnie-rastenija.ru/hamedoreja-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/hamerops-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/hatiora-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/havortija-uhod-v-domashnih-uslovijah-foto-vidov-s-nazvanijami/",
        "https://komnatnie-rastenija.ru/hirita-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/hlorofitum-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/hojja-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/homalomena-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/hoveja-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/jakobinija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/jatrofa-posadka-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/jeheverija-uhod-v-domashnih-uslovijah-razmnozhenie-listom-i-rozetkami-foto-vidov/",
        "https://komnatnie-rastenija.ru/jehmeja-uhod-v-domashnih-uslovijah-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/jeonium-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/jepifillum-uhod-v-domashnih-uslovijah-foto-vidov-razmnozhenie/",
        "https://komnatnie-rastenija.ru/jepipremnum-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/jepiscija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/jeshinantus-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/jeuharis-uhod-v-domashnih-uslovijah-foto-vidov-peresadka/",
        "https://komnatnie-rastenija.ru/jeustoma-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/jukka-domashnjaja-posadka-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/kaladium-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/kalanhoje-posadka-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/kalateja-uhod-v-domashnih-uslovijah-foto-i-nazvanija-vidov/",
        "https://komnatnie-rastenija.ru/kalceoljarija-posadka-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/kallistemon-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/kallizija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/kamelija-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/katarantus-posadka-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/kiparisovik-lavsona-uhod-v-domashnih-uslovijah-foto-i-opisanie/",
        "https://komnatnie-rastenija.ru/kislica-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/klerodendrum-uhod-v-domashnih-uslovijah-razmnozhenie-foto-vidov/",
        "https://komnatnie-rastenija.ru/klivija-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/kofejnoe-derevo-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/kolerija-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/koleus-posadka-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/kolumneja-uhod-v-domashnih-uslovijah-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/kordilina-uhod-v-domashnih-uslovijah-foto-vidy/",
        "https://komnatnie-rastenija.ru/kufeja-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/ledeburija-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/limonnoe-derevo-vyrashhivanie-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/litops-zhivoj-kamen-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/livistona-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/maranta-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/medinilla-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/mirt-uhod-v-domashnih-uslovijah-foto-komnatnogo-rastenija/",
        "https://komnatnie-rastenija.ru/molochaj-komnatnyj-uhod-v-domashnih-uslovijah-foto-i-nazvanija-vidov/",
        "https://komnatnie-rastenija.ru/molochaj-milja-uhod-v-domashnih-uslovijah-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/monstera-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/murajja-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/mushmula-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/nefrolepis-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/nematantus-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/nepentes-uhod-v-domashnih-uslovijah-foto-i-opisanie-hishhnogo-rastenija/",
        "https://komnatnie-rastenija.ru/nertera-posadka-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/opuncija-kaktus-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/orhideja-dendrobium-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/orhideja-kattleja-uhod-v-domashnih-uslovijah-peresadka-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/orhideja-vanda-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/pafiopedilum-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/pahipodium-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/pahira-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/pahistahis-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/paslen-komnatnyj-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/passiflora-vyrashhivanie-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/pedilantus-uhod-v-domashnih-uslovijah-razmnozhenie-foto-vidov/",
        "https://komnatnie-rastenija.ru/pelargonija-zonalnaja-uhod-v-domashnih-uslovijah-vyrashhivanie-iz-semjan/",
        "https://komnatnie-rastenija.ru/pelleja-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/peperomija-uhod-v-domashnih-uslovijah-foto-i-nazvanija-vidov/",
        "https://komnatnie-rastenija.ru/petrokosmeja-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/pileja-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/platicerium-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/pljumerija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/pljushh-uhod-v-domashnih-uslovijah-foto-vido/",
        "https://komnatnie-rastenija.ru/primula-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/puansettija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/roicissus-berezka-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/roza-domashnjaja-v-gorshke-uhod-vyrashhivanie-i-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/rozmarin-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/rujellija-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/sanhecija-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/sansevierija-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/scindapsus-uhod-v-domashnih-uslovijah-foto-vidov-razmnozhenie/",
        "https://komnatnie-rastenija.ru/selaginella-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/senpolija-uhod-v-domashnih-uslovijah-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/setkreazija-purpurnaja-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/shefflera-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/shljumbergera-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-sortov/",
        "https://komnatnie-rastenija.ru/singonium-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/sinningija-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/skutelljarija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/smitianta-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/solejrolija-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/spatifillum-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/sprekelija-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/stapelija-uhod-v-domashnih-uslovijah-foto-sortov/",
        "https://komnatnie-rastenija.ru/stefanotis-uhod-v-domashnih-uslovijah-foto-mozhno-li-derzhat-doma/",
        "https://komnatnie-rastenija.ru/streptokarpus-uhod-v-domashnih-uslovijah-vyrashhivanie-iz-semjan-foto/",
        "https://komnatnie-rastenija.ru/stromanta-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/strongilodon-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/tabernemontana-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/takka-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/tamarind-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/tespezija-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/tetrastigma-vuane-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/tideja-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov-i-sortov/",
        "https://komnatnie-rastenija.ru/tillandsija-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/titanopsis-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/tolstjanka-denezhnoe-derevo-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/tradeskancija-uhod-v-domashnih-uslovijah-razmnozhenie-foto-vidov/",
        "https://komnatnie-rastenija.ru/trahikarpus-forchuna-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/uajtfeldija-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/vallota-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/vashingtonija-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/vriezija-uhod-v-domashnih-uslovijah/",
        "https://komnatnie-rastenija.ru/zamiokulkas-v-domashnih-uslovijah-uhod-i-razmnozhenie-foto/",
        "https://komnatnie-rastenija.ru/zantedeskija-kalla-uhod-i-razmnozhenie-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/zefirantes-posadka-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/zhakaranda-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
        "https://komnatnie-rastenija.ru/zhasmin-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto/",
        "https://komnatnie-rastenija.ru/zhirjanka-vyrashhivanie-i-uhod-v-domashnih-uslovijah-foto-vidov/",
    ];
    let client = reqwest::Client::new();
    // Get title page
    // let url = "https://komnatnie-rastenija.ru/";
    // let response = client.get(url).send().await?;
    // let html = response.text().await?;
    // // Parse categories ('Рубрики')
    // let soup = soup::Soup::new(&html);
    // let urls = soup
    //     .class("cat-item")
    //     .find_all()
    //     .filter_map(|node| node.children().next())
    //     .filter_map(|node| node.get("href"))
    //     .collect::<Vec<String>>();
    // urls.iter().for_each(|f| println!("{:?}", f));

    // // Vec for plants urls
    // let mut plants_url: Vec<String> = Vec::new();
    // // For each category get all plants urls
    // for url in urls {
    //     // Get page count
    //     let response = client.get(&url).send().await?;
    //     let html = response.text().await?;
    //     let pages = page_count(&html);
    //     // Parse first page
    //     plants_url.append(&mut parse_titles(&html));
    //     // Parse rest pages
    //     for page in 1..pages {
    //         let url_page = url.clone() + "/page/" + page.to_string().as_str();

    //         let response = client.get(&url_page).send().await?;
    //         let html = response.text().await?;
    //         plants_url.append(&mut parse_titles(&html));
    //     }
    // }
    // // Remove duplicates
    // plants_url.sort_unstable();
    // plants_url.dedup();
    // dbg!(plants_url);
    // Vec for all plants info
    let mut plants_info: Vec<Houseplant> = Vec::with_capacity(plants_url.len());
    // Get info for each plants
    for url in plants_url {
        plants_info.push(
            parse_houseplant(&client, url)
                .await
                .expect("Can't get plant info"),
        );
    }
    Ok(plants_info)
}

fn parse_titles(html: &str) -> Vec<String> {
    let soup = soup::Soup::new(html);
    soup.tag("a")
        .attr("itemprop", "url")
        .find_all()
        .map(|a| a.get("href").unwrap())
        .collect::<Vec<String>>()
}

fn page_count(html: &str) -> usize {
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

async fn parse_houseplant(client: &reqwest::Client, url: &str) -> Option<Houseplant> {
    let response = client.get(url).send().await.ok()?;
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
    if let Some(node) = soup.tag("td").find_all().filter(|node| node.text().to_lowercase().contains("полив")).next() {
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
