
$version: "2.0"

namespace com.acme

@length(min: 8, max: 32)
string Password

@length(min: 6, max: 6)
@pattern("^[0-9]{6}$")
string MfaCode

@pattern("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$")
string Email


@mixin
structure UserPasword {
    @required
    password: Password,
}

@mixin
structure UserId {
    @required
    id: String,
}

@mixin
structure UserPublicInfo {
  @required
  name: String,
  @required
  email: Email,
}

structure UserInfo with [UserId, UserPublicInfo] {}

structure UserCreateInfo with [UserPublicInfo, UserPasword] {}

list UserList {
    member: UserInfo
}
