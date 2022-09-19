use octocrab::*;
pub fn build_octocrab(token: String) -> Octocrab {
    Octocrab::builder()
        .personal_token(token)
        .build()
        .expect("Unable to build Octocrab instance")
}
