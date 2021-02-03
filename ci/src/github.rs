use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
struct ActionContextPayload {
    event_name: String,
    //event: EventPayload,
}

pub async fn test_github(context_str: &str) {
    //let octocrab = octocrab::instance();
    //let issues = octocrab.issues(owner, repo);

    //let issue = match issues.get(13).await {
    //    Ok(issue) => issue,
    //    Err(err) => panic!("Error {:?}", err)
    //};

    //print!("{:#?}", issue);
    let context: ActionContextPayload = serde_json::from_str(&context_str).unwrap();


    print!("context_str: {}", context_str);
    print!("context: {:?}", context);
}
