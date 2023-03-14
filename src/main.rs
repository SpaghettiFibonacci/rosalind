use challenges::get_result;

pub mod challenges;
pub mod io;
fn main() {
    // get_result(challenges::dna::Dna);
    // get_result(challenges::rna::Rna);
    // get_result(challenges::revc::Revc);
    // get_result(challenges::hamm::Hamm);
    // get_result(challenges::perm::Perm);
    // get_result(challenges::iprb::Iprb);
    // get_result(challenges::gc::Gc);
    get_result(challenges::subs::Subs);
}
