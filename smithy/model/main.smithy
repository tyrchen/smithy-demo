$version: "2.0"

namespace com.acme

use aws.protocols#restJson1
use smithy.framework#ValidationException
use aws.api#service

/// Useres input
@service(sdkId: "user")
@restJson1
@httpBearerAuth
service UserService {
    version: "2023-12-03"
    resources: [User]
    operations: [Health, Signin]
}

@http(uri: "/health", method: "POST")
@auth([])
operation Health {
    input := {
        @required
        @httpHeader("x-message")
        message: String
    }
    output := {
        @required
        @httpHeader("x-status")
        message: String
    }
    errors: [ValidationException, ServerError]
}


/// Signin to get a token.
@http(uri: "/signin", method: "POST")
@auth([])
operation Signin {
    input := {
        @required
        username: String
        @required
        password: String
    }
    output := {
        @required
        accessToken: String
        @required
        refreshToken: String
        @required
        expiresIn: Long
    }
    errors: [ValidationException, ForbiddenError, ThrottlingError]
}
