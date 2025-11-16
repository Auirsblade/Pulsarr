# GroupAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createGroup**](GroupAPI.md#creategroup) | **POST** /group/create | Create Group
[**deleteGroup**](GroupAPI.md#deletegroup) | **DELETE** /group/delete/{id} | Delete group
[**getAllGroups**](GroupAPI.md#getallgroups) | **POST** /group/ | Get all groups
[**getMembershipTypes**](GroupAPI.md#getmembershiptypes) | **GET** /group/membershipTypes | Get the group membership types
[**getPrivacyTypes**](GroupAPI.md#getprivacytypes) | **GET** /group/privacyTypes | Get the group privacy types
[**getPulsarrGroup**](GroupAPI.md#getpulsarrgroup) | **GET** /group/{id} | Get a group by id
[**joinGroup**](GroupAPI.md#joingroup) | **POST** /group/join/{group_id} | Join group
[**leaveGroup**](GroupAPI.md#leavegroup) | **POST** /group/leave/{group_id} | Leave group
[**searchGroups**](GroupAPI.md#searchgroups) | **GET** /group/search | 
[**updateGroup**](GroupAPI.md#updategroup) | **POST** /group/update | Update group


# **createGroup**
```swift
    open class func createGroup(pulsarrApiKey: String, groupDTO: GroupDTO, completion: @escaping (_ data: GroupDTO?, _ error: Error?) -> Void)
```

Create Group

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)
let groupDTO = GroupDTO(pulsarrGroupId: 123, ratingSystemId: 123, name: "name_example", privacyType: "privacyType_example", ratingSystem: RatingSystemDTO(ratingSystemId: 123, masterRatingType: "masterRatingType_example", ratingMax: "ratingMax_example", name: "name_example", parameters: [RatingSystemParameterDTO(ratingSystemParameterId: 123, ratingSystemId: 123, parameterRatingMax: "parameterRatingMax_example", name: "name_example")]), createdByUserId: 123, creationDate: "creationDate_example", members: [GroupMemberDTO(pulsarrGroupId: 123, user: UserDTO(pulsarrUserId: 123, name: "name_example", email: "email_example", password: "password_example", joinDate: "joinDate_example"), groupRole: "groupRole_example", joinDate: "joinDate_example")]) // GroupDTO | 

// Create Group
GroupAPI.createGroup(pulsarrApiKey: pulsarrApiKey, groupDTO: groupDTO) { (response, error) in
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
 **groupDTO** | [**GroupDTO**](GroupDTO.md) |  | 

### Return type

[**GroupDTO**](GroupDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteGroup**
```swift
    open class func deleteGroup(id: Int, pulsarrApiKey: String, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Delete group

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let id = 987 // Int | 
let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)

// Delete group
GroupAPI.deleteGroup(id: id, pulsarrApiKey: pulsarrApiKey) { (response, error) in
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
 **pulsarrApiKey** | **String** | Pulsarr Api Key (user session key) | 

### Return type

**Bool**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllGroups**
```swift
    open class func getAllGroups(pulsarrApiKey: String, getRequest: GetRequest, completion: @escaping (_ data: [GroupDTO]?, _ error: Error?) -> Void)
```

Get all groups

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)
let getRequest = GetRequest(takeSize: 123) // GetRequest | 

// Get all groups
GroupAPI.getAllGroups(pulsarrApiKey: pulsarrApiKey, getRequest: getRequest) { (response, error) in
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
 **getRequest** | [**GetRequest**](GetRequest.md) |  | 

### Return type

[**[GroupDTO]**](GroupDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getMembershipTypes**
```swift
    open class func getMembershipTypes(completion: @escaping (_ data: [String]?, _ error: Error?) -> Void)
```

Get the group membership types

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient


// Get the group membership types
GroupAPI.getMembershipTypes() { (response, error) in
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
This endpoint does not need any parameter.

### Return type

**[String]**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPrivacyTypes**
```swift
    open class func getPrivacyTypes(completion: @escaping (_ data: [String]?, _ error: Error?) -> Void)
```

Get the group privacy types

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient


// Get the group privacy types
GroupAPI.getPrivacyTypes() { (response, error) in
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
This endpoint does not need any parameter.

### Return type

**[String]**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPulsarrGroup**
```swift
    open class func getPulsarrGroup(id: Int, pulsarrApiKey: String, completion: @escaping (_ data: GroupDTO?, _ error: Error?) -> Void)
```

Get a group by id

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let id = 987 // Int | 
let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)

// Get a group by id
GroupAPI.getPulsarrGroup(id: id, pulsarrApiKey: pulsarrApiKey) { (response, error) in
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
 **pulsarrApiKey** | **String** | Pulsarr Api Key (user session key) | 

### Return type

[**GroupDTO**](GroupDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **joinGroup**
```swift
    open class func joinGroup(groupId: Int, pulsarrApiKey: String, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Join group

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let groupId = 987 // Int | 
let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)

// Join group
GroupAPI.joinGroup(groupId: groupId, pulsarrApiKey: pulsarrApiKey) { (response, error) in
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
 **groupId** | **Int** |  | 
 **pulsarrApiKey** | **String** | Pulsarr Api Key (user session key) | 

### Return type

**Bool**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **leaveGroup**
```swift
    open class func leaveGroup(groupId: Int, pulsarrApiKey: String, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Leave group

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let groupId = 987 // Int | 
let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)

// Leave group
GroupAPI.leaveGroup(groupId: groupId, pulsarrApiKey: pulsarrApiKey) { (response, error) in
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
 **groupId** | **Int** |  | 
 **pulsarrApiKey** | **String** | Pulsarr Api Key (user session key) | 

### Return type

**Bool**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchGroups**
```swift
    open class func searchGroups(pulsarrApiKey: String, name: String? = nil, completion: @escaping (_ data: [GroupDTO]?, _ error: Error?) -> Void)
```



### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)
let name = "name_example" // String |  (optional)

GroupAPI.searchGroups(pulsarrApiKey: pulsarrApiKey, name: name) { (response, error) in
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
 **name** | **String** |  | [optional] 

### Return type

[**[GroupDTO]**](GroupDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateGroup**
```swift
    open class func updateGroup(pulsarrApiKey: String, groupDTO: GroupDTO, completion: @escaping (_ data: GroupDTO?, _ error: Error?) -> Void)
```

Update group

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let pulsarrApiKey = "pulsarrApiKey_example" // String | Pulsarr Api Key (user session key)
let groupDTO = GroupDTO(pulsarrGroupId: 123, ratingSystemId: 123, name: "name_example", privacyType: "privacyType_example", ratingSystem: RatingSystemDTO(ratingSystemId: 123, masterRatingType: "masterRatingType_example", ratingMax: "ratingMax_example", name: "name_example", parameters: [RatingSystemParameterDTO(ratingSystemParameterId: 123, ratingSystemId: 123, parameterRatingMax: "parameterRatingMax_example", name: "name_example")]), createdByUserId: 123, creationDate: "creationDate_example", members: [GroupMemberDTO(pulsarrGroupId: 123, user: UserDTO(pulsarrUserId: 123, name: "name_example", email: "email_example", password: "password_example", joinDate: "joinDate_example"), groupRole: "groupRole_example", joinDate: "joinDate_example")]) // GroupDTO | 

// Update group
GroupAPI.updateGroup(pulsarrApiKey: pulsarrApiKey, groupDTO: groupDTO) { (response, error) in
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
 **groupDTO** | [**GroupDTO**](GroupDTO.md) |  | 

### Return type

[**GroupDTO**](GroupDTO.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

