# Get All Vendors

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/vendors/all" method="get" summary="Get All Vendors" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authorizarion" type="string" %}
token from login
{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="integer" %}
pagination from 0
{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="integer" %}
limit of rows
{% endswagger-parameter %}

{% swagger-response status="200" description="" %}
```
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61525154347c025c4cdc29c0",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
                "vendor_address": "",
                "state": "",
                "city": "",
                "postal_code": "",
                "manufacturing_address": "",
                "vendor_email": "",
                "phone_numbers": "",
                "vendor_website": "",
                "created_at": "0001-01-01T00:00:00Z",
                "vendor_owner_name": "",
                "vendor_owner_phone_numbers": "",
                "vendor_owner_representative_name": "",
                "vendor_owner_email": "",
                "order_channel": "",
                "billing_conditions": "",
                "payment_terms": "",
                "currency": "",
                "user_type": "",
                "location": ""
            },
            {
                "_id": "619c9fc00db3748cdf2b63d2",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
                "vendor_address": "",
                "state": "",
                "city": "",
                "postal_code": "",
                "manufacturing_address": "",
                "vendor_email": "",
                "phone_numbers": "",
                "vendor_website": "",
                "created_at": "0001-01-01T00:00:00Z",
                "vendor_owner_name": "",
                "vendor_owner_phone_numbers": "",
                "vendor_owner_representative_name": "",
                "vendor_owner_email": "",
                "order_channel": "",
                "billing_conditions": "",
                "payment_terms": "",
                "currency": "",
                "user_type": "",
                "location": ""
            },
            {
                "_id": "619ce376f48fa28aa89367c0",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
                "vendor_address": "",
                "state": "",
                "city": "",
                "postal_code": "",
                "manufacturing_address": "",
                "vendor_email": "",
                "phone_numbers": "",
                "vendor_website": "",
                "created_at": "0001-01-01T00:00:00Z",
                "vendor_owner_name": "",
                "vendor_owner_phone_numbers": "",
                "vendor_owner_representative_name": "",
                "vendor_owner_email": "",
                "order_channel": "",
                "billing_conditions": "",
                "payment_terms": "",
                "currency": "",
                "user_type": "",
                "location": ""
            },
            {
                "_id": "619ce37cf48fa28aa89367c1",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
                "vendor_address": "",
                "state": "",
                "city": "",
                "postal_code": "",
                "manufacturing_address": "",
                "vendor_email": "",
                "phone_numbers": "",
                "vendor_website": "",
                "created_at": "0001-01-01T00:00:00Z",
                "vendor_owner_name": "",
                "vendor_owner_phone_numbers": "",
                "vendor_owner_representative_name": "",
                "vendor_owner_email": "",
                "order_channel": "",
                "billing_conditions": "",
                "payment_terms": "",
                "currency": "",
                "user_type": "",
                "location": ""
            },
            {
                "_id": "619ddabe971e7108b23f1c11",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
                "vendor_address": "",
                "state": "",
                "city": "",
                "postal_code": "",
                "manufacturing_address": "",
                "vendor_email": "",
                "phone_numbers": "",
                "vendor_website": "",
                "created_at": "0001-01-01T00:00:00Z",
                "vendor_owner_name": "",
                "vendor_owner_phone_numbers": "",
                "vendor_owner_representative_name": "",
                "vendor_owner_email": "",
                "order_channel": "",
                "billing_conditions": "",
                "payment_terms": "",
                "currency": "",
                "user_type": "",
                "location": ""
            },
            {
                "_id": "619ddd1ced772372b6cf504c",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
                "vendor_address": "",
                "state": "",
                "city": "",
                "postal_code": "",
                "manufacturing_address": "",
                "vendor_email": "",
                "phone_numbers": "",
                "vendor_website": "",
                "created_at": "0001-01-01T00:00:00Z",
                "vendor_owner_name": "",
                "vendor_owner_phone_numbers": "",
                "vendor_owner_representative_name": "",
                "vendor_owner_email": "",
                "order_channel": "",
                "billing_conditions": "",
                "payment_terms": "",
                "currency": "",
                "user_type": "",
                "location": ""
            },
            {
                "_id": "619ddd42ed772372b6cf504d",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
                "vendor_address": "",
                "state": "",
                "city": "",
                "postal_code": "",
                "manufacturing_address": "",
                "vendor_email": "",
                "phone_numbers": "",
                "vendor_website": "",
                "created_at": "0001-01-01T00:00:00Z",
                "vendor_owner_name": "",
                "vendor_owner_phone_numbers": "",
                "vendor_owner_representative_name": "",
                "vendor_owner_email": "",
                "order_channel": "",
                "billing_conditions": "",
                "payment_terms": "",
                "currency": "",
                "user_type": "",
                "location": ""
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400" description="" %}
```
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "error message will produce by server"
    }
}
```
{% endswagger-response %}

{% swagger-response status="401" description="" %}
```
{
    "responses": {
        "code": 401,
        "data": null,
        "isError": true,
        "message": "error message will produce by server"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request GET 'https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors/all?page=0&limit=10'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var requestOptions = {
  method: 'GET',
  redirect: 'follow'
};

fetch("https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors/all?page=0&limit=10", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors/all?page=0&limit=10",
  "method": "GET",
  "timeout": 0,
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}
