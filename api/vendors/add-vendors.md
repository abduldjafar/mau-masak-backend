# Add Vendors

{% swagger baseUrl="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/vendors" method="post" summary="Add new Vendors" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="body" name="" type="object" %}

{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
```
{
    "responses": {
        "code": 200,
        "data": null,
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="401" description="Could not find a cake matching this query." %}
```
{
    "code": 401,
    "data": null,
    "isError": true,
    "message": "token contains an invalid number of segments"
}
```
{% endswagger-response %}
{% endswagger %}

### Body Requests

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "brand_or_vendor": "manga asik",
    "HQ_country": "",
    "business_model": "",
    "phone_number": "",
    "email": "manga@asik.com",
    "address": "",
    "owner_name": "",
    "representative_name": "",
    "official_register_name": "",
    "vat_number": "",
    "bank_account": "",
    "certified_b_corporation": "",
    "category_or_products": ""
}
```
{% endtab %}

{% tab title="Descriptions" %}

{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request POST 'https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors' \
--header 'Content-Type: application/json' \
--data-raw '{
    "brand_or_vendor": "manga asik",
    "HQ_country": "",
    "business_model": "",
    "phone_number": "",
    "email": "manga@asik.com",
    "address": "",
    "owner_name": "",
    "representative_name": "",
    "official_register_name": "",
    "vat_number": "",
    "bank_account": "",
    "certified_b_corporation": "",
    "category_or_products": ""
}'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Content-Type", "application/json");

var raw = JSON.stringify({
  "brand_or_vendor": "manga asik",
  "HQ_country": "",
  "business_model": "",
  "phone_number": "",
  "email": "manga@asik.com",
  "address": "",
  "owner_name": "",
  "representative_name": "",
  "official_register_name": "",
  "vat_number": "",
  "bank_account": "",
  "certified_b_corporation": "",
  "category_or_products": ""
});

var requestOptions = {
  method: 'POST',
  headers: myHeaders,
  body: raw,
  redirect: 'follow'
};

fetch("https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/vendors",
  "method": "POST",
  "timeout": 0,
  "headers": {
    "Content-Type": "application/json"
  },
  "data": JSON.stringify({
    "brand_or_vendor": "manga asik",
    "HQ_country": "",
    "business_model": "",
    "phone_number": "",
    "email": "manga@asik.com",
    "address": "",
    "owner_name": "",
    "representative_name": "",
    "official_register_name": "",
    "vat_number": "",
    "bank_account": "",
    "certified_b_corporation": "",
    "category_or_products": ""
  }),
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}
