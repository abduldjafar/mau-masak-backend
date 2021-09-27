# Add Vendors

{% api-method method="post" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/vendors" %}
{% api-method-summary %}
Add new Vendors
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-body-parameters %}
{% api-method-parameter name="" type="object" required=true %}

{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```
{    "name": "Cake's name",    "recipe": "Cake's recipe name",    "cake": "Binary cake"}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=404 %}
{% api-method-response-example-description %}
Could not find a cake matching this query.
{% endapi-method-response-example-description %}

```
{    "message": "Ain't no cake like that."}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

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

