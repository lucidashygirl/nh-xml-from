use crate::*;

pub fn validate_dialogue_tree_config(config: &ConfigFile) -> String {
    let mut dialogue_vec: Vec<DialogueNode> = Vec::new();

    let name = match &config.name_field {
        Some(n) => n.to_string(),
        None => quit!("No name field found"),
    };

    match &config.dialogue_node {
        None => quit!("No dialogue nodes found"),
        Some(dialogue_node) => {
            for node in dialogue_node {
                let mut dialogue_nodes = DialogueNode::default();
                match node.get("name") {
                    None => quit!("No name in dialogue node"),
                    Some(n) => dialogue_nodes.name = n.as_str().unwrap().to_string(),
                }
                if let Some(conditions) = node.get("entry_condition") {
                    let mut parsed_conditions: Vec<String> = Vec::new();
                    #[allow(for_loops_over_fallibles)]
                    for condition in conditions.as_array() {
                        for c in condition {
                            parsed_conditions.push(c.as_str().unwrap().to_string());
                        }
                    }
                }
                if let Some(random) = node.get("randomize") {
                    dialogue_nodes.randomize = Some(random.as_bool().unwrap())
                }
                if let Some(dialogue) = node.get("dialogue") {
                    #[allow(for_loops_over_fallibles)]
                    for thing in dialogue.as_array() {
                        for d in thing {
                            let mut dialogue_vec: Vec<Dialogue> = Vec::new();
                            match d.get("page") {
                                None => quit!("No page"),
                                Some(pages) =>
                                {
                                    #[allow(for_loops_over_fallibles)]
                                    for page in pages.as_array() {
                                        let mut page_vec: Vec<String> = Vec::new();
                                        for p in page {
                                            page_vec.push(p.as_str().unwrap().to_string())
                                        }
                                        dialogue_vec.push(Dialogue { page: page_vec })
                                    }
                                }
                            }
                            dialogue_nodes.dialogue = Some(dialogue_vec);
                        }
                    }
                }
                if let Some(reveal_facts) = node.get("reveal_facts") {
                    let mut revealfact = RevealFacts::default();
                    #[allow(for_loops_over_fallibles)]
                    for facts in reveal_facts.as_array() {
                        for fact in facts {
                            match fact.get("fact_id") {
                                None => quit!("No fact id"),
                                Some(facts) =>
                                {
                                    #[allow(for_loops_over_fallibles)]
                                    for fact in facts.as_array() {
                                        let mut fact_vec: Vec<String> = Vec::new();
                                        for f in fact {
                                            fact_vec.push(f.as_str().unwrap().to_string())
                                        }
                                        revealfact.fact_id = fact_vec;
                                    }
                                }
                            }
                        }
                    }
                    dialogue_nodes.reveal_facts = Some(revealfact);
                }
                if let Some(spc) = node.get("set_persistent_condition") {
                    dialogue_nodes.set_persistent_condition =
                        Some(spc.as_str().unwrap().to_string());
                }
                if let Some(set_condition) = node.get("set_condition") {
                    #[allow(for_loops_over_fallibles)]
                    for conditions in set_condition.as_array() {
                        let mut cond: Vec<String> = Vec::new();
                        for condition in conditions {
                            cond.push(condition.as_str().unwrap().to_string())
                        }
                        dialogue_nodes.set_condition = Some(cond);
                    }
                }
                if let Some(dpc) = node.get("disable_persistent_condition") {
                    dialogue_nodes.disable_persistent_condition =
                        Some(dpc.as_str().unwrap().to_string());
                }
                if let Some(dialogue_target_ship_condition) =
                    node.get("dialogue_target_shiplog_condition")
                {
                    #[allow(for_loops_over_fallibles)]
                    for condition in dialogue_target_ship_condition.as_array() {
                        let mut cond_vec: Vec<String> = Vec::new();
                        for c in condition {
                            cond_vec.push(c.as_str().unwrap().to_string());
                        }
                        dialogue_nodes.dialogue_target_shiplog_condition = Some(cond_vec);
                    }
                }
                if let Some(dialogue_target) = node.get("dialogue_target") {
                    dialogue_nodes.dialogue_target =
                        Some(dialogue_target.as_str().unwrap().to_string());
                }
                if let Some(dialogue_options_list) = node.get("dialogue_options_list") {
                    let mut opt = DialogueOptionsList::default();
                    let mut list: Vec<DialogueOption> = Vec::new();
                    #[allow(for_loops_over_fallibles)]
                    for dialogue_options in dialogue_options_list.get("dialogue_option") {
                        #[allow(for_loops_over_fallibles)]
                        for options in dialogue_options.as_array() {
                            for option in options {
                                let mut dialogue_option = DialogueOption::default();
                                match option.get("text") {
                                    None => quit!("No text field in dialogue options"),
                                    Some(t) => {
                                        dialogue_option.text =
                                            t.as_str().unwrap().to_string().clone()
                                    }
                                }
                                match option.get("dialogue_target") {
                                    None => quit!("No target in dialogue options"),
                                    Some(t) => {
                                        dialogue_option.dialogue_target =
                                            Some(t.as_str().unwrap().to_string())
                                    }
                                }
                                if let Some(required_log_condition) =
                                    dialogue_options.get("required_log_condition")
                                {
                                    let mut conditions: Vec<String> = Vec::new();
                                    #[allow(for_loops_over_fallibles)]
                                    for log_condition in required_log_condition.as_array() {
                                        for cond in log_condition {
                                            conditions.push(cond.as_str().unwrap().to_string());
                                        }
                                    }
                                    dialogue_option.required_log_condition = Some(conditions);
                                }

                                if let Some(required_persistent_condition) =
                                    dialogue_options.get("required_persistent_condition")
                                {
                                    let mut conditions: Vec<String> = Vec::new();
                                    #[allow(for_loops_over_fallibles)]
                                    for log_condition in required_persistent_condition.as_array() {
                                        for cond in log_condition {
                                            conditions.push(cond.as_str().unwrap().to_string());
                                        }
                                    }
                                    dialogue_option.required_persistent_condition =
                                        Some(conditions);
                                }

                                if let Some(cancelled_persistent_condition) =
                                    dialogue_options.get("cancelled_persistent_condition")
                                {
                                    let mut conditions: Vec<String> = Vec::new();
                                    #[allow(for_loops_over_fallibles)]
                                    for log_condition in cancelled_persistent_condition.as_array() {
                                        for cond in log_condition {
                                            conditions.push(cond.as_str().unwrap().to_string());
                                        }
                                    }
                                    dialogue_option.cancelled_persistent_condition =
                                        Some(conditions);
                                }

                                if let Some(required_condition) =
                                    dialogue_options.get("required_condition")
                                {
                                    dialogue_option.required_condition =
                                        Some(required_condition.as_str().unwrap().to_string());
                                }
                                if let Some(cancelled_condition) =
                                    dialogue_options.get("cancelled_condition")
                                {
                                    dialogue_option.cancelled_condition =
                                        Some(cancelled_condition.as_str().unwrap().to_string());
                                }
                                if let Some(dialogue_target) =
                                    dialogue_options.get("dialogue_target")
                                {
                                    dialogue_option.dialogue_target =
                                        Some(dialogue_target.as_str().unwrap().to_string());
                                }
                                if let Some(condition_to_set) =
                                    dialogue_options.get("condition_to_set")
                                {
                                    dialogue_option.condition_to_set =
                                        Some(condition_to_set.as_str().unwrap().to_string());
                                }
                                if let Some(condition_to_cancel) =
                                    dialogue_options.get("condition_to_cancel")
                                {
                                    dialogue_option.condition_to_cancel =
                                        Some(condition_to_cancel.as_str().unwrap().to_string());
                                }
                                list.push(dialogue_option);
                            }
                        }
                    }
                    opt.dialogue_option = Some(list);
                    if let Some(reused_list) =
                        dialogue_options_list.get("reuse_dialogue_options_list_from")
                    {
                        opt.reuse_dialogue_options_list_from =
                            Some(reused_list.as_str().unwrap().to_string());
                    }
                    dialogue_nodes.dialogue_options_list = Some(opt);
                }
                dialogue_vec.push(dialogue_nodes);
            }
        }
    }

    generate_dialogue_tree_xml_string(config, (dialogue_vec, name))
}
pub fn generate_dialogue_tree_xml_string(
    toml: &ConfigFile,
    blocks: (Vec<DialogueNode>, String),
) -> String {
    let mut xml = String::new();
    let schema = match &toml.schema {
        Some(s) => s,
        None => &"https://raw.githubusercontent.com/Outer-Wilds-New-Horizons/new-horizons/main/NewHorizons/Schemas/dialogue_schema.xsd".to_owned(),
    };
    xml += format!(
        r#"<DialogueTree xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="{}">"#,
        schema
    )
    .as_str();
    xml += format!("<NameField>{}</NameField>", blocks.1).as_str();
    for block in blocks.0 {
        xml += "<DialogueNode>";
        xml += format!("<Name>{}</Name>", block.name).as_str();

        if let Some(condition) = block.entry_condition {
            for con in condition {
                xml += format!("<EntryCondition>{}</EntryCondition>", con).as_str()
            }
        }

        if let Some(random) = block.randomize {
            if random {
                xml += "<Randomize />"
            }
        };

        if let Some(dialogue) = block.dialogue {
            for mutter in dialogue {
                xml += "<Dialogue>";
                for p in mutter.page {
                    xml += format!("<Page>{}</Page>", p).as_str()
                }
                xml += "</Dialogue>";
            }
        }

        if let Some(reveal_facts) = block.reveal_facts {
            xml += "<RevealFacts>";
            for id in reveal_facts.fact_id {
                xml += format!("<FactID>{}</FactID>", id).as_str()
            }
            xml += "</RevealFacts>";
        }

        if let Some(condition) = block.set_persistent_condition {
            xml += format!(
                "<SetPersistentCondition>{}</SetPersistentCondition>",
                condition
            )
            .as_str();
        }

        if let Some(set_condition) = block.set_condition {
            for condition in set_condition {
                xml += format!("<SetCondition>{}</SetCondition>", condition).as_str();
            }
        }

        if let Some(condition) = block.disable_persistent_condition {
            xml += format!(
                "<DisablePersistentCondition>{}</DisablePersistentCondition>",
                condition
            )
            .as_str();
        }

        if let Some(conditions) = block.dialogue_target_shiplog_condition {
            for condition in conditions {
                xml += format!(
                    "<DialogueTargetShipLogCondition>{}</DialogueTargetShipLogCondition>",
                    condition
                )
                .as_str();
            }
        }

        if let Some(target) = block.dialogue_target {
            xml += format!("<DialogueTarget>{}</DialogueTarget>", target).as_str();
        }

        if let Some(dialogue_options_list) = block.dialogue_options_list {
            xml += "<DialogueOptionsList>";
            if let Some(dialogue_option) = dialogue_options_list.dialogue_option {
                for option in dialogue_option {
                    xml += "<DialogueOption>";
                    if let Some(required_log_condition) = option.required_log_condition {
                        for condition in required_log_condition {
                            xml += format!(
                                "<RequiredLogCondition>{}</RequiredLogCondition>",
                                condition
                            )
                            .as_str()
                        }
                    }

                    if let Some(required_persistent_condition) =
                        option.required_persistent_condition
                    {
                        for condition in required_persistent_condition {
                            xml += format!(
                                "<RequiredPersistentCondition>{}</RequiredPersistentCondition>",
                                condition
                            )
                            .as_str()
                        }
                    }

                    if let Some(cancelled_persistent_condition) =
                        option.cancelled_persistent_condition
                    {
                        for condition in cancelled_persistent_condition {
                            xml += format!(
                                "<CancelledPersistentCondition>{}</CancelledPersistentCondition>",
                                condition
                            )
                            .as_str()
                        }
                    }

                    if let Some(required_condition) = option.required_condition {
                        xml += format!(
                            "<RequiredCondition>{}</RequiredCondition>",
                            required_condition
                        )
                        .as_str()
                    }

                    if let Some(cancelled_condition) = option.cancelled_condition {
                        xml += format!(
                            "<CancelledCondition>{}</CancelledCondition>",
                            cancelled_condition
                        )
                        .as_str()
                    }

                    // Required field, no check needed
                    xml += format!("<Text>{}</Text>", option.text).as_str();

                    if let Some(dialogue_target) = option.dialogue_target {
                        xml +=
                            format!("<DialogueTarget>{}</DialogueTarget>", dialogue_target).as_str()
                    }

                    if let Some(condition) = option.condition_to_set {
                        xml += format!("<ConditionToSet>{}</ConditionToSet>", condition).as_str()
                    }

                    if let Some(condition) = option.condition_to_cancel {
                        xml +=
                            format!("<ConditionToCancel>{}</ConditionToCancel>", condition).as_str()
                    }

                    xml += "</DialogueOption>"
                }
            }
            if let Some(from) = dialogue_options_list.reuse_dialogue_options_list_from {
                xml += format!(
                    "<ReuseDialogueOptionsListFrom>{}</ReuseDialogueOptionsListFrom>",
                    from
                )
                .as_str()
            }
            xml += "</DialogueOptionsList>"
        }

        xml += "</DialogueNode>";
    }
    xml += "</DialogueTree>";
    xml
}
