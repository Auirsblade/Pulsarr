# AuthAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**signin**](AuthAPI.md#signin) | **POST** /auth/signin | Sign in


# **signin**
```swift
    open class func signin(signInRequest: SignInRequest, completion: @escaping (_ data: SignInResponse?, _ error: Error?) -> Void)
```

Sign in

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import PulsarrClient

let signInRequest = SignInRequest(username: "username_example", password: "password_example", hwKey: "hwKey_example") // SignInRequest | 

// Sign in
AuthAPI.signin(signInRequest: signInRequest) { (response, error) in
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
 **signInRequest** | [**SignInRequest**](SignInRequest.md) |  | 

### Return type

[**SignInResponse**](SignInResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

