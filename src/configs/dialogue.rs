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
            // Make sure there is a default entry condition present
            let mut entry_conditions: Vec<&str> = Vec::new();
            for (root_loops, node) in dialogue_node.iter().enumerate() {
                if let Some(conditions) = node.get("entry_condition") {
                    #[allow(for_loops_over_fallibles)]
                    for condition in conditions.as_array() {
                        for c in condition {
                            let cond = match c.as_str() {
                                Some(c) => c,
                                None => quit!(format!("Invalid type for dialogue_node[{}].entry_condition, expected String", root_loops)),
                            };
                            entry_conditions.push(cond);
                        }
                    }
                }
            }
            if !entry_conditions.contains(&"DEFAULT") {
                quit!("No default entry condition")
            }

            for (root_loops, node) in dialogue_node.iter().enumerate() {
                let mut dialogue_nodes = DialogueNode::default();
                match node.get("name") {
                    None => quit!(format!(
                        "dialogue_node[{}].name does not exist.",
                        root_loops
                    )),
                    Some(n) => {
                        dialogue_nodes.name = match n.as_str() {
                            Some(n) => n.to_owned(),
                            None => quit!(format!(
                                "Invalid type for dialogue_node[{}].name, expected String",
                                root_loops
                            )),
                        }
                    }
                }
                if let Some(conditions) = node.get("entry_condition") {
                    let mut parsed_conditions: Vec<String> = Vec::new();
                    #[allow(for_loops_over_fallibles)]
                    for condition in conditions.as_array() {
                        for (local_loop, c) in condition.iter().enumerate() {
                            let cond = match c.as_str() {
                                Some(c) => c.to_owned(),
                                None => quit!(format!("Invalid type for dialogue_node[{}].entry_condition[{}], expected String", root_loops, local_loop)),
                            };
                            parsed_conditions.push(cond);
                        }
                    }
                    dialogue_nodes.entry_condition = Some(parsed_conditions);
                }
                if let Some(random) = node.get("randomize") {
                    dialogue_nodes.randomize = match random.as_bool() {
                        Some(r) => Some(r),
                        None => quit!(format!(
                            "Invalid type for dialogue_node[{}].randomize, expected Boolean",
                            root_loops
                        )),
                    }
                }
                if let Some(dialogue) = node.get("dialogue") {
                    #[allow(for_loops_over_fallibles)]
                    for thing in dialogue.as_array() {
                        for (loops, d) in thing.iter().enumerate() {
                            let mut dialogue_vec: Vec<Dialogue> = Vec::new();
                            match d.get("page") {
                                None => {
                                    quit!(format!(
                                        "dialogue_node[{}].dialogue[{}].page not found",
                                        root_loops, loops
                                    ))
                                }
                                Some(pages) =>
                                {
                                    #[allow(for_loops_over_fallibles)]
                                    for page in pages.as_array() {
                                        let mut page_vec: Vec<String> = Vec::new();
                                        for (local_loop, p) in page.iter().enumerate() {
                                            let ver_page = match p.as_str() {
                                                Some(p) => p.to_owned(),
                                                None => {
                                                    quit!(format!("Invalid type for dialogue_node[{}].dialogue[{}].page[{}], expected String.", root_loops, loops, local_loop))
                                                }
                                            };
                                            page_vec.push(ver_page)
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
                                None => quit!(format!(
                                    "dialogue_node[{}].reveal_facts.fact_id not found",
                                    root_loops
                                )),
                                Some(facts) =>
                                {
                                    #[allow(for_loops_over_fallibles)]
                                    for fact in facts.as_array() {
                                        let mut fact_vec: Vec<String> = Vec::new();
                                        for (loops, f) in fact.iter().enumerate() {
                                            let fact_ver = match f.as_str() {
                                                Some(f) => f.to_owned(),
                                                None => {
                                                    quit!(format!("Invalid type for dialogue_node[{}].reveal_facts.fact_id[{}], expected String.", root_loops, loops))
                                                }
                                            };
                                            fact_vec.push(fact_ver)
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
                    dialogue_nodes.set_persistent_condition = match spc.as_str() {
                        Some(s) => Some(s.to_owned()),
                        None => quit!(format!("Invalid type for dialogue_node[{}].set_persistent_condition, expected String", root_loops)),
                    }
                }
                if let Some(set_condition) = node.get("set_condition") {
                    #[allow(for_loops_over_fallibles)]
                    for conditions in set_condition.as_array() {
                        let mut cond: Vec<String> = Vec::new();
                        for (loops, condition) in conditions.iter().enumerate() {
                            let condition_ok = match condition.as_str() {
                                Some(c) => c.to_owned(),
                                None => {
                                    quit!(format!("Invalid type for dialogue_node[{}].set_condition[{}], expected String.", root_loops, loops))
                                }
                            };
                            cond.push(condition_ok)
                        }
                        dialogue_nodes.set_condition = Some(cond);
                    }
                }
                if let Some(dpc) = node.get("disable_persistent_condition") {
                    dialogue_nodes.disable_persistent_condition = match dpc.as_str() {
                        Some(d) => Some(d.to_owned()),
                        None => quit!(format!("Invalid type for dialogue_node[{}].disable_persistent_condition, expected String", root_loops))
                    }
                }
                if let Some(dialogue_target_ship_condition) =
                    node.get("dialogue_target_shiplog_condition")
                {
                    #[allow(for_loops_over_fallibles)]
                    for condition in dialogue_target_ship_condition.as_array() {
                        let mut cond_vec: Vec<String> = Vec::new();
                        for (loops, c) in condition.iter().enumerate() {
                            let condition_ok = match c.as_str() {
                                Some(c) => c.to_owned(),
                                None => {
                                    quit!(format!("Invalid type for dialogue_node[{}].dialogue_target_shiplog_condition[{}], expected String.", root_loops, loops))
                                }
                            };
                            cond_vec.push(condition_ok);
                        }
                        dialogue_nodes.dialogue_target_shiplog_condition = Some(cond_vec);
                    }
                }
                if let Some(dialogue_target) = node.get("dialogue_target") {
                    dialogue_nodes.dialogue_target = match dialogue_target.as_str() {
                        Some(d) => Some(d.to_owned()),
                        None => {
                            quit!(format!("Invalid type for dialogue_node[{}].dialogue_target, expected String", root_loops))
                        }
                    }
                }
                if let Some(dialogue_options_list) = node.get("dialogue_options_list") {
                    let mut opt = DialogueOptionsList::default();
                    let mut list: Vec<DialogueOption> = Vec::new();
                    #[allow(for_loops_over_fallibles)]
                    for dialogue_options in dialogue_options_list.get("dialogue_option") {
                        #[allow(for_loops_over_fallibles)]
                        for options in dialogue_options.as_array() {
                            for (loops, option) in options.iter().enumerate() {
                                let mut dialogue_option = DialogueOption::default();
                                match option.get("text") {
                                    None => quit!(format!("dialogue_node[{}].dialogue_options_list.dialogue_option[{}].text not found", root_loops, loops)),
                                    Some(t) => {
                                        dialogue_option.text = match t.as_str() {
                                            Some(o) => o.to_owned().clone(),
                                            None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].text, expected String", root_loops, loops)),
                                        }
                                    }
                                }
                                match option.get("dialogue_target") {
                                    None => quit!(format!("dialogue_node[{}].dialogue_options_list.dialogue_option[{}].dialogue_target not found", root_loops, loops)),
                                    Some(t) => {
                                        dialogue_option.dialogue_target = match t.as_str() {
                                            Some(o) => Some(o.to_owned()),
                                            None => quit!(format!("Invalid type for dialogue_node.dialogue_options_list.dialogue_option[{}].dialogue_target, expected String", loops))
                                        }
                                    }
                                }
                                if let Some(required_log_condition) =
                                    dialogue_options.get("required_log_condition")
                                {
                                    let mut conditions: Vec<String> = Vec::new();
                                    #[allow(for_loops_over_fallibles)]
                                    for log_condition in required_log_condition.as_array() {
                                        for (local_loop, cond) in log_condition.iter().enumerate() {
                                            let condition_ok = match cond.as_str() {
                                                Some(c) => c.to_owned(),
                                                None => {
                                                    quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].required_log_condition[{}], expected String.", root_loops, loops, local_loop))
                                                }
                                            };
                                            conditions.push(condition_ok);
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
                                        for (local_loop, cond) in log_condition.iter().enumerate() {
                                            let condition_ok = match cond.as_str() {
                                                Some(c) => c.to_owned(),
                                                None => {
                                                    quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].required_persistent_condition[{}], expected String.", root_loops, loops, local_loop))
                                                }
                                            };
                                            conditions.push(condition_ok);
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
                                        for (local_loop, cond) in log_condition.iter().enumerate() {
                                            let condition_ok = match cond.as_str() {
                                                Some(c) => c.to_owned(),
                                                None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].cancelled_persistent_condition[{}], expected String", root_loops, loops, local_loop))
                                            };
                                            conditions.push(condition_ok);
                                        }
                                    }
                                    dialogue_option.cancelled_persistent_condition =
                                        Some(conditions);
                                }

                                if let Some(required_condition) =
                                    dialogue_options.get("required_condition")
                                {
                                    dialogue_option.required_condition =
                                        match required_condition.as_str() {
                                            Some(r) => Some(r.to_owned()),
                                            None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].required_condition, expected String", root_loops, loops)),
                                        };
                                }
                                if let Some(cancelled_condition) =
                                    dialogue_options.get("cancelled_condition")
                                {
                                    dialogue_option.cancelled_condition =
                                        match cancelled_condition.as_str() {
                                            Some(c) => Some(c.to_owned()),
                                            None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].cancelled_condition, expected String", root_loops, loops))
                                        };
                                }
                                if let Some(dialogue_target) =
                                    dialogue_options.get("dialogue_target")
                                {
                                    dialogue_option.dialogue_target =
                                        match dialogue_target.as_str() {
                                            Some(c) => Some(c.to_owned()),
                                            None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].dialogue_target, expected String", root_loops, loops))
                                        };
                                }
                                if let Some(condition_to_set) =
                                    dialogue_options.get("condition_to_set")
                                {
                                    dialogue_option.condition_to_set =
                                        match condition_to_set.as_str() {
                                            Some(c) => Some(c.to_owned()),
                                            None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].condition_to_set, expected String", root_loops, loops))
                                        };
                                }
                                if let Some(condition_to_cancel) =
                                    dialogue_options.get("condition_to_cancel")
                                {
                                    dialogue_option.condition_to_cancel =
                                        match condition_to_cancel.as_str() {
                                            Some(c) => Some(c.to_owned()),
                                            None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.dialogue_option[{}].condition_to_cancel, expected String", root_loops, loops))
                                        };
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
                        match reused_list.as_str() {
                            Some(c) => Some(c.to_owned()),
                            None => quit!(format!("Invalid type for dialogue_node[{}].dialogue_options_list.reuse_dialogue_options_list_from, expected String", root_loops))
                        };
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

        if let Some(conditions) = block.entry_condition {
            for condition in conditions {
                xml += format!("<EntryCondition>{}</EntryCondition>", condition).as_str()
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
