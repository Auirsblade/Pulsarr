# MusicBrainzAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**getArtist**](MusicBrainzAPI.md#getartist) | **GET** /musicbrainz/artist/{mbid} | 
[**getRecording**](MusicBrainzAPI.md#getrecording) | **GET** /musicbrainz/recording/{mbid} | 
[**getRelease**](MusicBrainzAPI.md#getrelease) | **GET** /musicbrainz/release/{mbid} | 
[**searchArtists**](MusicBrainzAPI.md#searchartists) | **GET** /musicbrainz/search/artist | 
[**searchRecordings**](MusicBrainzAPI.md#searchrecordings) | **GET** /musicbrainz/search/recording | 
[**searchReleases**](MusicBrainzAPI.md#searchreleases) | **GET** /musicbrainz/search/release | 


# **getArtist**
```swift
    open class func getArtist(mbid: String, completion: @escaping (_ data: Artist?, _ error: Error?) -> Void)
```



Get artist by MBID

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let mbid = "mbid_example" // String | 

MusicBrainzAPI.getArtist(mbid: mbid) { (response, error) in
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
 **mbid** | **String** |  | 

### Return type

[**Artist**](Artist.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRecording**
```swift
    open class func getRecording(mbid: String, completion: @escaping (_ data: Recording?, _ error: Error?) -> Void)
```



Get recording by MBID

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let mbid = "mbid_example" // String | 

MusicBrainzAPI.getRecording(mbid: mbid) { (response, error) in
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
 **mbid** | **String** |  | 

### Return type

[**Recording**](Recording.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getRelease**
```swift
    open class func getRelease(mbid: String, completion: @escaping (_ data: Release?, _ error: Error?) -> Void)
```



Get release by MBID

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let mbid = "mbid_example" // String | 

MusicBrainzAPI.getRelease(mbid: mbid) { (response, error) in
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
 **mbid** | **String** |  | 

### Return type

[**Release**](Release.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchArtists**
```swift
    open class func searchArtists(query: String, limit: Int? = nil, offset: Int? = nil, completion: @escaping (_ data: ArtistSearchResponse?, _ error: Error?) -> Void)
```



Search for artists on MusicBrainz

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let query = "query_example" // String | 
let limit = 987 // Int |  (optional)
let offset = 987 // Int |  (optional)

MusicBrainzAPI.searchArtists(query: query, limit: limit, offset: offset) { (response, error) in
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
 **query** | **String** |  | 
 **limit** | **Int** |  | [optional] 
 **offset** | **Int** |  | [optional] 

### Return type

[**ArtistSearchResponse**](ArtistSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchRecordings**
```swift
    open class func searchRecordings(query: String, limit: Int? = nil, offset: Int? = nil, completion: @escaping (_ data: RecordingSearchResponse?, _ error: Error?) -> Void)
```



Search for recordings on MusicBrainz

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let query = "query_example" // String | 
let limit = 987 // Int |  (optional)
let offset = 987 // Int |  (optional)

MusicBrainzAPI.searchRecordings(query: query, limit: limit, offset: offset) { (response, error) in
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
 **query** | **String** |  | 
 **limit** | **Int** |  | [optional] 
 **offset** | **Int** |  | [optional] 

### Return type

[**RecordingSearchResponse**](RecordingSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **searchReleases**
```swift
    open class func searchReleases(query: String, limit: Int? = nil, offset: Int? = nil, completion: @escaping (_ data: ReleaseSearchResponse?, _ error: Error?) -> Void)
```



Search for releases on MusicBrainz

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let query = "query_example" // String | 
let limit = 987 // Int |  (optional)
let offset = 987 // Int |  (optional)

MusicBrainzAPI.searchReleases(query: query, limit: limit, offset: offset) { (response, error) in
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
 **query** | **String** |  | 
 **limit** | **Int** |  | [optional] 
 **offset** | **Int** |  | [optional] 

### Return type

[**ReleaseSearchResponse**](ReleaseSearchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

