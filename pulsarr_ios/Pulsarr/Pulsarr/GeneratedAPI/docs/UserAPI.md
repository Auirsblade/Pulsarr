# UserAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addUser**](UserAPI.md#adduser) | **POST** /user/add | Add user
[**deleteUser**](UserAPI.md#deleteuser) | **DELETE** /user/delete/{id} | Delete user
[**getAllUsers**](UserAPI.md#getallusers) | **GET** /user/ | Get all users
[**getCurrentUser**](UserAPI.md#getcurrentuser) | **GET** /user/currentUser | Get a user by active session
[**getPulsarrUser**](UserAPI.md#getpulsarruser) | **GET** /user/{id} | Get a user by id
[**updateUser**](UserAPI.md#updateuser) | **POST** /user/update | Update user


# **addUser**
```swift
    open class func addUser(userDTO: UserDTO, completion: @escaping (_ data: UserDTO?, _ error: Error?) -> Void)
```

Add user

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let userDTO = UserDTO(pulsarrUserId: 123, name: "name_example", email: "email_example", password: "password_example", joinDate: "joinDate_example") // UserDTO | 

// Add user
UserAPI.addUser(userDTO: userDTO) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userDTO** | [**UserDTO**](UserDTO.md) |  | 

### Return type

[**UserDTO**](UserDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteUser**
```swift
    open class func deleteUser(id: Int, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Delete user

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let id = 987 // Int | 

// Delete user
UserAPI.deleteUser(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **Int** |  | 

### Return type

**Bool**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllUsers**
```swift
    open class func getAllUsers(pulsarrApiKey: String, completion: @escaping (_ data: [UserDTO]?, _ error: Error?) -> Void)
```

Get all users

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)

// Get all users
UserAPI.getAllUsers(pulsarrApiKey: pulsarrApiKey) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pulsarrApiKey** | **String** | Pulsarr Api Key (user session key) | 

### Return type

[**[UserDTO]**](UserDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getCurrentUser**
```swift
    open class func getCurrentUser(pulsarrApiKey: String, completion: @escaping (_ data: UserDTO?, _ error: Error?) -> Void)
```

Get a user by active session

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)

// Get a user by active session
UserAPI.getCurrentUser(pulsarrApiKey: pulsarrApiKey) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **pulsarrApiKey** | **String** | Pulsarr Api Key (user session key) | 

### Return type

[**UserDTO**](UserDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPulsarrUser**
```swift
    open class func getPulsarrUser(id: Int, completion: @escaping (_ data: UserDTO?, _ error: Error?) -> Void)
```

Get a user by id

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let id = 987 // Int | 

// Get a user by id
UserAPI.getPulsarrUser(id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **Int** |  | 

### Return type

[**UserDTO**](UserDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateUser**
```swift
    open class func updateUser(userDTO: UserDTO, completion: @escaping (_ data: UserDTO?, _ error: Error?) -> Void)
```

Update user

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let userDTO = UserDTO(pulsarrUserId: 123, name: "name_example", email: "email_example", password: "password_example", joinDate: "joinDate_example") // UserDTO | 

// Update user
UserAPI.updateUser(userDTO: userDTO) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **userDTO** | [**UserDTO**](UserDTO.md) |  | 

### Return type

[**UserDTO**](UserDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

