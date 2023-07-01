pub trait Catalog<C: Class> {
    fn query_by_id(&self, id: &str) -> Option<&C>;
    fn query_by_department(&self, code: &str) -> Vec<&C>;
}

pub trait Class {
    /// Returns the full id of the class.  
    /// CS 115, MA 121, ACC 200, etc.
    fn id(&self) -> String;
    /// Returns the department code for the class **as uppercase**.  
    /// MA, CS, ACC, etc.
    fn department(&self) -> String;
    /// Returns the full department name corresponding to the department code.  
    /// Mathematics, Computer Science, Accounting, etc.
    fn department_name(&self) -> String;
    /// Returns the number code that comes after the department code.
    fn discriminator(&self) -> u8;
    /// Returns the title of the class, **excluding the ID**.
    fn title(&self) -> String;
    /// Returns the description of the class.
    fn description(&self) -> String;
    /// Returns the number of credits the class provides.
    fn credits(&self) -> u8;
    /// Returns the prerequisites of a class formatted as a single String.
    fn prerequisites(&self) -> String;
    /// Returns a Vec of the semesters that the class is offered in.
    fn offered(&self) -> Vec<String>;
    /// Returns a Vec of the IDs of any cross listed classes.
    fn cross_listings(&self) -> Vec<String>;
    /// Returns a Vec of the distributions that the class belongs to.
    fn distributions(&self) -> Vec<String>;
    /// Returns a url pointing to an online entry for the class.
    fn url(&self) -> String;
}
