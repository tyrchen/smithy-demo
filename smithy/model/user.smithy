$version: "2.0"

namespace com.acme

use smithy.framework#ValidationException

resource User {
  identifiers: { id: String }
  create: CreateUser,
  read: GetUser,
}

@http(uri: "/users", method: "POST", code: 201)
@auth([])
operation CreateUser {
  input := {
    @required
    name: String,
    @required
    email: String,
    @required
    password: String,
  }
  output := {
    @httpHeader("x-user-id")
    @required
    id: String,
  }
  errors: [ValidationException, ThrottlingError, ServerError]
}

@readonly
@http(uri: "/users/{id}", method: "GET", code: 200)
operation GetUser {
  input := {
    @required
    @httpLabel
    id: String,
  }
  output: UserInfo,
  errors: [ValidationException, ThrottlingError, ServerError]
}

structure UserInfo {
  @required
  id: String,
  @required
  name: String,
  @required
  email: String,
}
