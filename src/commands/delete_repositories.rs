use crate::DeleteRepositories;

pub fn exec(token: String, args: DeleteRepositories) {
    println!("Deleting Repositories: {}, {}", token, args.file);
}
