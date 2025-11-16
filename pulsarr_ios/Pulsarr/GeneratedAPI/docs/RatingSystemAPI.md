# RatingSystemAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addRatingSystem**](RatingSystemAPI.md#addratingsystem) | **POST** /rating-system/add | Add rating system
[**addRatingSystemParameter**](RatingSystemAPI.md#addratingsystemparameter) | **POST** /rating-system/parameter/add | Add rating system parameter
[**deleteRatingSystem**](RatingSystemAPI.md#deleteratingsystem) | **DELETE** /rating-system/delete/{id} | Delete rating system
[**deleteRatingSystemParameter**](RatingSystemAPI.md#deleteratingsystemparameter) | **DELETE** /rating-system/parameter/delete/{id} | Delete rating system parameter
[**getAllRatingSystemParameters**](RatingSystemAPI.md#getallratingsystemparameters) | **GET** /rating-system/parameter | Get all rating system parameters
[**getAllRatingSystems**](RatingSystemAPI.md#getallratingsystems) | **GET** /rating-system/ | Get all rating systems
[**getRatingSystem**](RatingSystemAPI.md#getratingsystem) | **GET** /rating-system/{id} | Get a rating system by id
[**getRatingSystemParameter**](RatingSystemAPI.md#getratingsystemparameter) | **GET** /rating-system/parameter/{id} | Get a rating system parameter by id
[**getRatingTypes**](RatingSystemAPI.md#getratingtypes) | **GET** /rating-system/ratingTypes | Get the master rating types
[**updateRatingSystem**](RatingSystemAPI.md#updateratingsystem) | **POST** /rating-system/update | Update rating system
[**updateRatingSystemParameter**](RatingSystemAPI.md#updateratingsystemparameter) | **POST** /rating-system/parameter/update | Update rating system parameter


# **addRatingSystem**
```swift
    open class func addRatingSystem(ratingSystem: RatingSystem, completion: @escaping (_ data: RatingSystem?, _ error: Error?) -> Void)
```

Add rating system

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let ratingSystem = RatingSystem(ratingSystemId: 123, masterRatingType: "masterRatingType_example", ratingMax: "ratingMax_example", name: "name_example") // RatingSystem | 

// Add rating system
RatingSystemAPI.addRatingSystem(ratingSystem: ratingSystem) { (response, error) in
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
 **ratingSystem** | [**RatingSystem**](RatingSystem.md) |  | 

### Return type

[**RatingSystem**](RatingSystem.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addRatingSystemParameter**
```swift
    open class func addRatingSystemParameter(ratingSystemParameter: RatingSystemParameter, completion: @escaping (_ data: RatingSystemParameter?, _ error: Error?) -> Void)
```

Add rating system parameter

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let ratingSystemParameter = RatingSystemParameter(ratingSystemParameterId: 123, ratingSystemId: 123, parameterRatingMax: "parameterRatingMax_example", name: "name_example") // RatingSystemParameter | 

// Add rating system parameter
RatingSystemAPI.addRatingSystemParameter(ratingSystemParameter: ratingSystemParameter) { (response, error) in
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
 **ratingSystemParameter** | [**RatingSystemParameter**](RatingSystemParameter.md) |  | 

### Return type

[**RatingSystemParameter**](RatingSystemParameter.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteRatingSystem**
```swift
    open class func deleteRatingSystem(id: Int, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Delete rating system

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Delete rating system
RatingSystemAPI.deleteRatingSystem(id: id) { (response, error) in
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

# **deleteRatingSystemParameter**
```swift
    open class func deleteRatingSystemParameter(id: Int, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Delete rating system parameter

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Delete rating system parameter
RatingSystemAPI.deleteRatingSystemParameter(id: id) { (response, error) in
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

# **getAllRatingSystemParameters**
```swift
    open class func getAllRatingSystemParameters(completion: @escaping (_ data: [RatingSystemParameter]?, _ error: Error?) -> Void)
```

Get all rating system parameters

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient


// Get all rating system parameters
RatingSystemAPI.getAllRatingSystemParameters() { (response, error) in
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

[**[RatingSystemParameter]**](RatingSystemParameter.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllRatingSystems**
```swift
    open class func getAllRatingSystems(completion: @escaping (_ data: [RatingSystem]?, _ error: Error?) -> Void)
```

Get all rating systems

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient


// Get all rating systems
RatingSystemAPI.getAllRatingSystems() { (response, error) in
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

[**[RatingSystem]**](RatingSystem.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRatingSystem**
```swift
    open class func getRatingSystem(id: Int, completion: @escaping (_ data: RatingSystem?, _ error: Error?) -> Void)
```

Get a rating system by id

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Get a rating system by id
RatingSystemAPI.getRatingSystem(id: id) { (response, error) in
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

[**RatingSystem**](RatingSystem.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRatingSystemParameter**
```swift
    open class func getRatingSystemParameter(id: Int, completion: @escaping (_ data: RatingSystemParameter?, _ error: Error?) -> Void)
```

Get a rating system parameter by id

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Get a rating system parameter by id
RatingSystemAPI.getRatingSystemParameter(id: id) { (response, error) in
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

[**RatingSystemParameter**](RatingSystemParameter.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRatingTypes**
```swift
    open class func getRatingTypes(completion: @escaping (_ data: [String]?, _ error: Error?) -> Void)
```

Get the master rating types

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient


// Get the master rating types
RatingSystemAPI.getRatingTypes() { (response, error) in
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

# **updateRatingSystem**
```swift
    open class func updateRatingSystem(ratingSystem: RatingSystem, completion: @escaping (_ data: RatingSystem?, _ error: Error?) -> Void)
```

Update rating system

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let ratingSystem = RatingSystem(ratingSystemId: 123, masterRatingType: "masterRatingType_example", ratingMax: "ratingMax_example", name: "name_example") // RatingSystem | 

// Update rating system
RatingSystemAPI.updateRatingSystem(ratingSystem: ratingSystem) { (response, error) in
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
 **ratingSystem** | [**RatingSystem**](RatingSystem.md) |  | 

### Return type

[**RatingSystem**](RatingSystem.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateRatingSystemParameter**
```swift
    open class func updateRatingSystemParameter(ratingSystemParameter: RatingSystemParameter, completion: @escaping (_ data: RatingSystemParameter?, _ error: Error?) -> Void)
```

Update rating system parameter

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let ratingSystemParameter = RatingSystemParameter(ratingSystemParameterId: 123, ratingSystemId: 123, parameterRatingMax: "parameterRatingMax_example", name: "name_example") // RatingSystemParameter | 

// Update rating system parameter
RatingSystemAPI.updateRatingSystemParameter(ratingSystemParameter: ratingSystemParameter) { (response, error) in
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
 **ratingSystemParameter** | [**RatingSystemParameter**](RatingSystemParameter.md) |  | 

### Return type

[**RatingSystemParameter**](RatingSystemParameter.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

