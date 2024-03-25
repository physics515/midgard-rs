use serde::{Deserialize, Serialize};

use crate::HeightDate;

/*
*** ChurnsList Scheme ***

{
		"churns": [
				{
						"height": "2000000",
						"date": "946684801000000000"
				}
		]
}
*/

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChurnsList(Option<Vec<HeightDate>>);

impl ChurnsList {
	#[must_use]
	pub const fn new(churns: Option<Vec<HeightDate>>) -> Self {
		Self(churns)
	}

	#[must_use]
	pub const fn get_churns(&self) -> &Option<Vec<HeightDate>> {
		&self.0
	}
}
