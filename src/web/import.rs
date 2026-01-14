use scraper::{Html, Selector};
use reqwest::header::USER_AGENT;
use reqwest::blocking::Client;


pub(crate) fn import(pnum:&u32) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://www.acmicpc.net/problem/{}", pnum);
    let client = Client::new();
    
    let response = client.get(&url)
        .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()?;

    let document = Html::parse_document(&response.text()?);

    // 2. 각 섹션별 선택자를 정의합니다.
    let selectors = [
        ("설명", "#problem_description"),
        ("입력", "#problem_input"),
        ("출력", "#problem_output"),
    ];
            println!("

");
    // 3. 데이터를 추출하고 출력합니다.
    for (label, selector_str) in selectors.iter() {
        let selector = Selector::parse(selector_str).unwrap();

        if let Some(element) = document.select(&selector).next() {
            // 내부 텍스트를 수집하고 공백을 정리합니다.
            let text: String = element.text().collect::<Vec<_>>().concat();
            println!("[{}] 
{}", label, text.trim());
        }

            println!("

");
    }
    
        
    Ok(())

}

