use std::collections::HashMap;

pub struct App {
    /// the currently being edited json key.
    pub(crate) key_input: String,

    /// the currently being edited json value.
    pub(crate) value_input: String,

    /// The representation of our key and value pairs with serde Serialize support
    pub(crate) pairs: HashMap<String, String>,

    /// the current screen the user is looking at, and will later determine what is rendered.
    pub(crate) current_screen: CurrentScreen,

    /// the optional state containing which of the key or value pair the user is editing.
    /// It is an option, because when the user is not directly editing a key-value pair,
    /// this will be set to `None`.
    pub(crate) currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        Self {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    /// This function will be called when the user saves a key-value pair in the editor.
    /// It adds the two stored variables to the key-value pairs HashMap,
    /// and resets the status of all of the editing variables.
    pub fn set_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    /// This function is checking if something is currently being edited,
    /// and if it is, swapping between editing the Key and Value fields.
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }

    pub fn key_input(&self) -> &str {
        &self.key_input
    }

    pub fn value_input(&self) -> &str {
        &self.value_input
    }

    pub fn pairs(&self) -> &HashMap<String, String> {
        &self.pairs
    }

    pub fn current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }

    pub fn currently_editing(&self) -> Option<&CurrentlyEditing> {
        self.currently_editing.as_ref()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}
