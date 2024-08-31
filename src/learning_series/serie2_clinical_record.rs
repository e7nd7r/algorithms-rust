// monilitic example of clinical record.

pub struct ClinicalRecord {
    pub patient_name: String,
    pub patient_last_name: String,
    pub patient_age: String,

    pub clinical_history: Vec<(String, i32, String)>, // diagnostic, diagnistic_age, status  
    pub diagnostics: Vec<(String, String, String)>, // where (Date, Description, Clasiffication),
    pub prescriptions: Vec<(String, String, String)>, // where (Data, Prescription, Dosis)
}

impl ClinicalRecord {
    pub fn add_clinical_history(&mut self, diagnostic: &str, diagnostic_age: i32, status: &str) {
        self.clinical_history.push((diagnostic.to_owned(), diagnostic_age, status.to_owned()))
    }

    pub fn add_diagnostic(&mut self, date: &str, description: &str, classification: &str) {
        self.diagnostics.push((date.to_owned(), description.to_owned(), classification.to_owned()));
    }

    pub fn add_prescription(&mut self, date: &str, prescription: &str, dosis: &str) {
        self.prescriptions.push((date.to_owned(), prescription.to_owned(), dosis.to_owned()));
    }
}