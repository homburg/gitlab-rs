extern crate rest_client;
extern crate serialize;

use rest_client::RestClient;
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
}

#[deriving(Encodable,Decodable,Show)]
struct MergeRequest {
	id: int,
	title: String,
	description: String,
}

    println!("{}", RestClient::get("").unwrap());

    // You can use an array of tuples to create a GET with query parameters.
    // The client handles all the URL-encoding and escaping for you.

    println!("{}", RestClient::get_with_params("http://example.com/resource", 
                                               [("id", "50"), ("foo", "bar")]).unwrap());

    // You can also use an array of tuples to create a POST with form parameters. 
    // The client sets the content-type to application/x-www-form-urlencoded for you.

    println!("{}", RestClient::post_with_params("http://example.com/resource",
                                                [("param1", "one"), 
                                                 ("param2", "two")]).unwrap());

    // You can POST a string or a JSON object with just a string and a MIME type.
    println!("{}", RestClient::post("http://example.com/resource",
                                    json::encode(&object).as_slice(), 
                                    "application/json").unwrap());

    // PUT and PATCH are supported as well, just like POST.

    // You can delete a resource with a simple DELETE. delete_with_params works too.

    println!("{}", RestClient::delete("http://example.com/resource").unwrap());

    /*
      The response struct has a few fields
      code (a simple integer)
      body (a string)
      status (a typed response code, from Hyper)
      headers (typed headers from Hyper)
    */

    let response = RestClient::get("http://example.com/resource").unwrap();

    println!("{:d}", response.code); // -> 404

    for header in response.headers.iter() {
        println!("{}", header); // -> (Cache-Control, max-age=604800) ...
    }

    println!("{}", response.to_string());                 

    /*
      All of the underlying errors are passed up through 
      the RestError struct in the Result.

      pub enum RestError {
        UrlParseError(ParseError),
        HttpRequestError(HttpError),
        HttpIoError(IoError)
      }
    */
}
