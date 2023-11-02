use serde_json::json;
use anki_assistant::anki_connect_wrapper::AnkiClient;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Create an AnkiClient instance
    let anki_client = AnkiClient::new("http://192.168.31.162:8765");

    // let demo_payload = demo();
    // anki_client.create_note_template(&demo_payload).await?;

    let word_template_payload = create_note_template("word", 5, 5);
    anki_client.create_note_template(&word_template_payload).await?;
    Ok(())    
}

fn demo() -> serde_json::Value {
    let model_fields = json!([
        "word",
        "context1",
        "context1_eg1",
        "context1_eg1_explain",
    ]);
    let model_templates = json!([
        {
            "Name": "Card 1",
            "Front": "{{Front}}",
            "Back": "{{Back}}"
        }
    ]);
    
    json!({
        "action": "createModel",
        "version": 6,
        "params": {
            "modelName": "TestModel",
            "inOrderFields": model_fields,
            "cardTemplates": model_templates
        }
    })

}

fn create_note_template(name: &str, max_context: i8, max_eg_percontext: i8) -> serde_json::Value {
    let (template_fields_array, card_templates_array) = (1..=max_context).fold(
        (
            vec![String::from("word"), String::from("word_summury")],
            Vec::new()
        ),
        |(mut fields, mut cards), i| {
            fields.push(format!("context{}", i));
            let (inner_fields, inner_cards ) = (1..=max_eg_percontext).fold(
                (Vec::new(), Vec::new()),
                |(mut inner_fields, mut inner_cards), j| {
                    inner_fields.push(format!("context{}_eg{}", i, j));
                    inner_fields.push(format!("context{}_eg{}_explain", i, j));
                    inner_cards.push(json!({
                        "Name": format!("{}.Card.{}-{}", name, i, j),
                        "Front": format!("{{{{#context{}_eg{}}}}}\n{{{{context{}_eg{}}}}}\n{{{{/context{}_eg{}}}}}", i, j, i, j, i, j),
                        "Back": format!("{{{{context{}_eg{}}}}}\n{{{{word_summury}}}}\n{{{{context{}}}}}\n{{{{context{}_eg{}_explain}}}}", i, j , i, i, j)
                    }));
                    (inner_fields, inner_cards)
                }
            );
            fields.extend(inner_fields);
            cards.extend(inner_cards);
            (fields, cards)
        }
    );


    json!({
        "action": "createModel",
        "version": 6,
        "params": {
            "modelName": name,
            "inOrderFields": json!(template_fields_array),
            "cardTemplates": json!(card_templates_array)
        }
    })
}