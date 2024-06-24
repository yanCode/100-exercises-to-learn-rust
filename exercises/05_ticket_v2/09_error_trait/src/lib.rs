#![allow(dead_code)]

use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum TicketNewError {
  TitleError(String),
  DescriptionError(String),
}

impl Display for TicketNewError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Title cannot be empty")
  }
}

// impl std::error::Error for TicketNewError {}

#[derive(Clone)]
enum Status {
  ToDo,
  InProgress { assigned_to: String },
  Done,
}

struct Ticket {
  title: String,
  description: String,
  status: Status,
}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
  match Ticket::new(title.clone(), description.clone(), status.clone()) {
    Ok(ticket) => ticket,
    Err(err) => match err {
      TicketNewError::TitleError(_) => panic!("{err}"),
      TicketNewError::DescriptionError(_) => {
        Ticket::new(title, "Description not provided".to_string(), status).unwrap()
      }
    }
  }
}

impl Ticket {
  pub fn new(title: String, description: String, status: Status) -> Result<Ticket, TicketNewError> {
    if title.is_empty() {
      return Err(TicketNewError::TitleError("Ticket cannot be empty".to_string()));
    }
    
    if title.len() > 50 {
      return Err(TicketNewError::TitleError(
        "Title cannot be longer than 50 bytes".to_string(),
      ));
    }
    if description.is_empty() {
      return Err(TicketNewError::DescriptionError(
        "Description cannot be empty".to_string(),
      ));
    }
    if description.len() > 500 {
      return Err(TicketNewError::DescriptionError(
        "Description cannot be longer than 500 bytes".to_string(),
      ));
    }
    
    Ok(Ticket { title, description, status })
  }
}


#[cfg(test)]
mod tests {
  use common::{overly_long_description, valid_description, valid_title};
  use crate::{easy_ticket, Status};
  
  #[test]
  #[should_panic(expected = "Title cannot be empty")]
  fn ticket_cannot_be_empty() {
    easy_ticket("".into(), valid_description(), Status::ToDo);
  }
  
  #[test]
  fn template_description_is_used_if_too_long() {
    let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
    assert_eq!(ticket.description, "Description not provided");
  }
}