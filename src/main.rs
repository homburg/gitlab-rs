extern crate rest_client;
extern crate serialize;

use rest_client::RestClient;
use std::os;
use serialize::json;

//    "description": null,
//    "default_branch": "master",
//    "public": false,
//    "visibility_level": 0,
//    "ssh_url_to_repo": "git@example.com:diaspora/diaspora-client.git",
//    "http_url_to_repo": "http://example.com/diaspora/diaspora-client.git",
//    "web_url": "http://example.com/diaspora/diaspora-client",
//    "owner": {
//      "id": 3,
//      "name": "Diaspora",
//      "created_at": "2013-09-30T13: 46: 02Z"
//    },
//    "name": "Diaspora Client",
//    "name_with_namespace": "Diaspora / Diaspora Client",
//    "path": "diaspora-client",
//    "path_with_namespace": "diaspora/diaspora-client",
//    "issues_enabled": true,
//    "merge_requests_enabled": true,
//    "wiki_enabled": true,
//    "snippets_enabled": false,
//    "created_at": "2013-09-30T13: 46: 02Z",
//    "last_activity_at": "2013-09-30T13: 46: 02Z",
//    "namespace": {
//      "created_at": "2013-09-30T13: 46: 02Z",
//      "description": "",
//      "id": 3,
//      "name": "Diaspora",
//      "owner_id": 1,
//      "path": "diaspora",
//      "updated_at": "2013-09-30T13: 46: 02Z"
//    },
//    "archived": false
//  },

#[deriving(Encodable,Decodable,Show)]
struct Project {
	id: int,
	name: String,
	path: String,
}

#[deriving(Encodable,Decodable,Show)]
struct MergeRequest {
	id: int,
	title: String,
	description: String,
}


fn main() {
	let args = os::args();

	let cmds = args.tail();

	let cmd = if cmds.len() > 0 {
		cmds[0].clone()
	} else {
		"".to_string()
	};


	if cmd.as_slice() == "projects" {
		println!("Listing projects...");
	};

	if cmd.as_slice() == "merge-requests" {
		println!("Listing merge requests...");
	};

	let mut i: int = 0;
	for a in cmds.iter() {
		println!("{} {}", i, a);
		i += 1;
	};

	let private_token = match os::getenv("GITLAB_PRIVATE_TOKEN") {
		Some(token) => token,
		None => fail!("You should specify a GITLAB_PRIVATE_TOKEN"),
	};

	let gitlab_url = match os::getenv("GITLAB_URL") {
		Some(url) => url,
		None => fail!("You should specify a GITLAB_URL"),
	};

    let projects_response = RestClient::get_with_params(
		format!("{}/api/v3/projects", gitlab_url).as_slice(),
		[
			("private_token", private_token.as_slice()),
			("state", "opened"),
		],
	).unwrap();

    if projects_response.code != 200 {
		fail!("Oops {}", projects_response.code);
	};

	let projects: Vec<Project> = json::decode(projects_response.body.as_slice()).unwrap();

	for project in projects.iter() {
		println!("{path}", path=project.path);
	}
}
