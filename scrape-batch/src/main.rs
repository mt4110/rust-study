use std::env;
use easy_scraper::Pattern;

// args flat 
// args detached
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = &args[1];
    
    // let selector1 = scraper::Selector::parse("dt").unwrap();
    // let selector2 = scraper::Selector::parse("dd").unwrap();
    let pat = Pattern::new(
        r#"
<div class="dottable-line">
    <dl>
        <dt class="dottable-vm">{{PropertyNameTitle}}</dt>
        <dd class="dottable-vm">{{PropertyName}}</dd>
    </dl>
</div>
<div class="dottable-line">
<dl>
    <dt class="dottable-vm">{{SalePriceTitle}}</dt>
    <dd class="dottable-vm">
        <span class="dottable-value">{{SalePrice}}</span>
    </dd>
    </dl>
</div>
<div class="dottable-line">
<dl>
    <dt>{{AddressTitlt}}</dt>
    <dd>{{Address}}</dd>
</dl>
<dl>
    <dt>{{NearStationTitlt}}</dt>
    <dd>{{NearStation}}</dd>
</dl>
</div>
"#,
    )
    .unwrap();
    let client = reqwest::blocking::Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/69.0.3497.100")
    .build()
    .unwrap();

let doc = client
    .get(url)
    .send()
    .unwrap()
    .text()
    .unwrap();

let ms = pat.matches(&doc);
println!("{:#?}", ms);

    Ok(())
}