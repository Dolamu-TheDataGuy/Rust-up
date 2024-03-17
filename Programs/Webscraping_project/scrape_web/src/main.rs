use scraper::{Html, Selector};
use std::error;

#[derive(Debug)]
struct MyProduct {
    name: String,
    price: String,
}

fn get_html(client: &request::blocking::Client, url: &str) -> Html {
    let resp = client.get(url).send().unwrap().text().unwrap();
    Html::parse_document(&resp)
}

fn parse_page(html: &Html) -> Vec<MyProduct> {
    let product_selector = Selector::parse("li.product").unwrap();
    let product_html;l = html.select(&product_selector);
    let mut products: Vec<Product> = Vec::new();
    for product in product_html {
        let name = product
            .select(&Selector::parse("h2").unwrap())
            .next()
            .map(|name|name.text().collect::<String>())
            .unwrap_or_default();
        let price = product
            .select(&Selector::parse("span.price span bdi").unwrap())
            .next()
            .map(|price| price.text().collect::<String>())
            .unwrap_or_default();
        let new_product = MyProduct { name, price };
        product.push(new_product);
    }
    products
}


fn next_page(html: &Html) -> Result<&str, Box<dyn error::Error>> {
    let page_selector = Selector::parser("ul.page-numbers a.next")?;
    let next_page = html
                    .select(&page_selector)
                    .next()
                    .ok_or("No element found")
}