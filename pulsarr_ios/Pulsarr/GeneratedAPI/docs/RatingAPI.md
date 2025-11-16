# RatingAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addRating**](RatingAPI.md#addrating) | **POST** /rating/add | Add rating
[**addRatingDetail**](RatingAPI.md#addratingdetail) | **POST** /rating/rating_detail/add | Add rating detail
[**deleteRating**](RatingAPI.md#deleterating) | **DELETE** /rating/delete/{id} | Delete rating
[**deleteRatingDetail**](RatingAPI.md#deleteratingdetail) | **DELETE** /rating/rating_detail/delete/{id} | Delete rating detail
[**getAllRatingDetails**](RatingAPI.md#getallratingdetails) | **GET** /rating/rating_detail | Get all rating details
[**getAllRatings**](RatingAPI.md#getallratings) | **GET** /rating/ | Get all ratings
[**getRating**](RatingAPI.md#getrating) | **GET** /rating/{id} | Get a rating by id
[**getRatingDetail**](RatingAPI.md#getratingdetail) | **GET** /rating/rating_detail/{id} | Get a rating detail by id
[**updateRating**](RatingAPI.md#updaterating) | **POST** /rating/update | Update rating
[**updateRatingDetail**](RatingAPI.md#updateratingdetail) | **POST** /rating/rating_detail/update | Update rating detail


# **addRating**
```swift
    open class func addRating(rating: Rating, completion: @escaping (_ data: Rating?, _ error: Error?) -> Void)
```

Add rating

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let rating = Rating(ratingId: 123, pulsarrUserId: 123, pulsarrGroupId: 123, ratingSystemId: 123, comments: "comments_example", ratingValue: "ratingValue_example") // Rating | 

// Add rating
RatingAPI.addRating(rating: rating) { (response, error) in
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
 **rating** | [**Rating**](Rating.md) |  | 

### Return type

[**Rating**](Rating.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **addRatingDetail**
```swift
    open class func addRatingDetail(ratingDetail: RatingDetail, completion: @escaping (_ data: RatingDetail?, _ error: Error?) -> Void)
```

Add rating detail

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let ratingDetail = RatingDetail(ratingDetailId: 123, ratingId: 123, ratingSystemParameterId: 123, ratingValue: "ratingValue_example") // RatingDetail | 

// Add rating detail
RatingAPI.addRatingDetail(ratingDetail: ratingDetail) { (response, error) in
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
 **ratingDetail** | [**RatingDetail**](RatingDetail.md) |  | 

### Return type

[**RatingDetail**](RatingDetail.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteRating**
```swift
    open class func deleteRating(id: Int, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Delete rating

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Delete rating
RatingAPI.deleteRating(id: id) { (response, error) in
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

# **deleteRatingDetail**
```swift
    open class func deleteRatingDetail(id: Int, completion: @escaping (_ data: Bool?, _ error: Error?) -> Void)
```

Delete rating detail

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Delete rating detail
RatingAPI.deleteRatingDetail(id: id) { (response, error) in
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

# **getAllRatingDetails**
```swift
    open class func getAllRatingDetails(completion: @escaping (_ data: [RatingDetail]?, _ error: Error?) -> Void)
```

Get all rating details

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient


// Get all rating details
RatingAPI.getAllRatingDetails() { (response, error) in
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

[**[RatingDetail]**](RatingDetail.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getAllRatings**
```swift
    open class func getAllRatings(completion: @escaping (_ data: [Rating]?, _ error: Error?) -> Void)
```

Get all ratings

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient


// Get all ratings
RatingAPI.getAllRatings() { (response, error) in
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

[**[Rating]**](Rating.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRating**
```swift
    open class func getRating(id: Int, completion: @escaping (_ data: Rating?, _ error: Error?) -> Void)
```

Get a rating by id

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Get a rating by id
RatingAPI.getRating(id: id) { (response, error) in
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

[**Rating**](Rating.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRatingDetail**
```swift
    open class func getRatingDetail(id: Int, completion: @escaping (_ data: RatingDetail?, _ error: Error?) -> Void)
```

Get a rating detail by id

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let id = 987 // Int | 

// Get a rating detail by id
RatingAPI.getRatingDetail(id: id) { (response, error) in
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

[**RatingDetail**](RatingDetail.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateRating**
```swift
    open class func updateRating(rating: Rating, completion: @escaping (_ data: Rating?, _ error: Error?) -> Void)
```

Update rating

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let rating = Rating(ratingId: 123, pulsarrUserId: 123, pulsarrGroupId: 123, ratingSystemId: 123, comments: "comments_example", ratingValue: "ratingValue_example") // Rating | 

// Update rating
RatingAPI.updateRating(rating: rating) { (response, error) in
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
 **rating** | [**Rating**](Rating.md) |  | 

### Return type

[**Rating**](Rating.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateRatingDetail**
```swift
    open class func updateRatingDetail(ratingDetail: RatingDetail, completion: @escaping (_ data: RatingDetail?, _ error: Error?) -> Void)
```

Update rating detail

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let ratingDetail = RatingDetail(ratingDetailId: 123, ratingId: 123, ratingSystemParameterId: 123, ratingValue: "ratingValue_example") // RatingDetail | 

// Update rating detail
RatingAPI.updateRatingDetail(ratingDetail: ratingDetail) { (response, error) in
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
 **ratingDetail** | [**RatingDetail**](RatingDetail.md) |  | 

### Return type

[**RatingDetail**](RatingDetail.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

