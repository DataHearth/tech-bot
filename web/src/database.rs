use polodb_core::{
    bson::{doc, Regex},
    Database,
};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Technology {
    pub link: String,
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthorizedUser {
    discord_id: String,
}

pub struct DB {
    db: Database,
}

impl DB {
    pub fn new() -> Self {
        Self {
            db: Database::open_file(std::env::var("DB_PATH").expect("missing DB_PATH"))
                .expect("failed to initialize database"),
        }
    }

    /// Add a new technology to the database.
    pub fn add_tech(&self, name: &str, link: &str, tags: &[&str]) -> Result<(), Error> {
        self.db
            .collection::<Technology>("technologies")
            .insert_one(Technology {
                link: link.into(),
                name: name.into(),
                tags: tags.iter().map(|s| s.to_string()).collect(),
            })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(())
    }

    pub fn remove_tech(&self, name: String) -> Result<(), Error> {
        self.db
            .collection::<Technology>("technologies")
            .delete_one(doc! { "name": name })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(())
    }

    pub fn list_tech(&self) -> Result<Vec<Technology>, Error> {
        Ok(self
            .db
            .collection("technologies")
            .find(None)
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .filter(|doc| doc.is_ok())
            .map(|doc| doc.unwrap())
            .collect())
    }

    pub fn search_tech(
        &self,
        name: String,
        options: String,
        tags: &[&str],
    ) -> Result<Vec<Technology>, Error> {
        Ok(self
            .db
            .collection::<Technology>("technologies")
            .find(doc! { "name": {"$regex": Regex {
                pattern: name,
                options: options,
            }}, "tags": {
                "$in": tags
            } })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .map(
                |doc| doc.unwrap(), // todo: find a way to handle error
            )
            .collect::<Vec<Technology>>())
    }

    pub fn set_auth_user(&self, discord_id: String) -> Result<(), Error> {
        self.db
            .collection("authorized_users")
            .insert_one(AuthorizedUser { discord_id })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?;

        Ok(())
    }

    pub fn is_auth_user(&self, discord_id: String) -> Result<bool, Error> {
        Ok(self
            .db
            .collection::<AuthorizedUser>("authorized_users")
            .find_one(doc! { "discord_id": discord_id })
            .map_err(|err| Error::new(ErrorKind::InvalidInput, err))?
            .is_some())
    }
}
