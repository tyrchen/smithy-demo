$version: "2.0"

namespace com.acme

use smithy.framework#ValidationException

resource User {
  identifiers: { id: String }
  create: CreateUser,
  read: GetUser,
  list: ListUsers,
  update: UpdateUser,
  delete: DeleteUser,
  operations: [GetUserByEmail, ChangePassword]
}

@requestCompression(
    encodings: ["gzip"]
)
@http(uri: "/users", method: "POST", code: 201)
@auth([])
operation CreateUser {
  input: UserCreateInfo
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

@readonly
@paginated(inputToken: "nextToken", outputToken: "nextToken",
           pageSize: "size", items: "users")
@http(uri: "/users", method: "GET", code: 200)
operation ListUsers {
  input := {
    @httpQuery("limit")
    size: Integer = 50,
    @httpQuery("nextToken")
    nextToken: String,
  }
  output := {
    @required
    users: UserList,
    nextToken: String,
  }
  errors: [ValidationException, ThrottlingError, ServerError]
}

@idempotent
@http(uri: "/users/{id}", method: "PUT", code: 200)
operation UpdateUser {
  input := {
    @required
    @httpLabel
    id: String,
    name: String,
    email: String,
  }
  output := {}
  errors: [ValidationException, ThrottlingError, UnauthorizedError, ServerError]
}

@idempotent
@http(uri: "/users/{id}", method: "DELETE", code: 204)
operation DeleteUser {
  input := {
    @required
    @httpLabel
    id: String,
  }
  output := {}
  errors: [ValidationException, ThrottlingError, UnauthorizedError, ServerError]
}

@http(uri: "/users/{id}/password", method: "PATCH", code: 200)
operation ChangePassword {
  input := {
    @required
    @httpLabel
    id: String,
    @required
    oldPassword: Password,
    @required
    mfaCode: MfaCode,
    @required
    newPassword: Password,
  }
  output := {}
  errors: [ValidationException, ThrottlingError, UnauthorizedError, ServerError]
}

@readonly
@http(uri: "/user", method: "GET", code: 200)
operation GetUserByEmail {
  input := {
    @required
    @httpQuery("email")
    id: String,
  }
  output: UserInfo,
  errors: [ValidationException, ThrottlingError, ServerError]
}
