
fn main() {
 
  // Variables can be type annotated.
    let cabin: bool = true;

    let bitcoin: f64 = 1.008;  // Regular annotation
    let day   = 16i32;         // Suffix annotation

    // Or a default will be used.
    let fare   = 139.0; // `f64`
    let pincode = 560032;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; 
    inferred_type = 4294967296i64;  // Type i64 is inferred from another line
    

  }