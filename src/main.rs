extern crate http;
extern crate url;
extern crate serialize;

use http::client::RequestWriter;
use http::method::Get;
use url::Url;
use std::os;

use serialize::json;

// {
//    "id": 4,
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
}

#[deriving(Encodable,Decodable,Show)]
struct MergeRequest {
	id: int,
	title: String,
	description: String,
}

fn get_gitlab_items(url_str: String) -> String {
    let url = Url::parse(url_str.as_slice()).unwrap();
    let request: RequestWriter = match RequestWriter::new(Get, url) {
        Ok(request) => request,
        Err(error) => fail!(":-( {}", error),
    };

    let mut response = match request.read_response() {
        Ok(response) => response,
        Err((_request, error)) => fail!(":-( {}", error),
    };

	let response_body = match response.read_to_string() {
		Ok(body) => body,
		Err(error) => fail!(":-( {}", error),
	};

	return response_body;
}

fn get_projects(url_str: String) -> Vec<Project> {
	let response_body = get_gitlab_items(url_str);

	let projects: Vec<Project> = json::decode(response_body.as_slice()).unwrap();
	return projects;
}

fn get_merge_requests(url_str: String) -> Vec<MergeRequest> {
	let response_body = get_gitlab_items(url_str);

	let mrs = json::decode(response_body.as_slice()).unwrap();
	return mrs;
}

fn main() {
	let private_token = match os::getenv("GITLAB_PRIVATE_TOKEN") {
		Some(token) => token,
		None => fail!("You should probably specifiy a token"),
	};

	let gitlab_url = match os::getenv("GITLAB_URL") {
		Some(url) => url,
		None => fail!("You should specifiy a gitlab url"),
	};

	let base_url = gitlab_url;
	let api_base = "/api/v3";

	let projects = get_projects(
		format!("{}{}/projects?private_token={}", base_url, api_base, private_token),
	);

	let mut educas = &Project { name: "Ukendt".to_string(), id: 13 };

	for p in projects.iter() {
		println!("{}", p.name);
		if p.name.as_slice() == "Educas.com" {
			educas = p;
		}
	};

	println!("{}", educas);

	let mrs = get_merge_requests(
		format!(
			"{}{}/projects/{}/merge_requests?private_token={}&state=opened",
			base_url,
			api_base,
			educas.id,
			private_token,
		)
	);

	for m in mrs.iter() {
		let description_format = match m.description.as_slice() {
			"" => "".to_string(),
			s => format!("\n===\n{}\n===\n", s),
		};

		println!("## {}\n{}", m.title, description_format);
	}
}
