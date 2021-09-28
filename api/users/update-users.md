# Update Users

{% api-method method="put" host="https://mico-backend-services-i5jta7dz4q-uc.a.run.app" path="/v1/users" %}
{% api-method-summary %}
Update Users
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authentication" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-body-parameters %}
{% api-method-parameter name="body" type="object" required=false %}
all datas should be send
{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```javascript
{
    "responses": {
        "code": 200,
        "data": null,
        "isError": false,
        "message": "success"
    }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=400 %}
{% api-method-response-example-description %}
Could not find a cake matching this query.
{% endapi-method-response-example-description %}

```javascript
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

### Body Request

{% tabs %}
{% tab title="Json Bodys" %}
```javascript
{
    "name": "koteka",
    "surname": "aspsasa",
    "birthdate": "2021-09-22T03:23:23.275+00:00",
    "mobile_number": "",
    "address_street": "",
    "address_number": "",
    "address_postal_code": "",
    "address_country": "",
    "address_city": ""

}
```
{% endtab %}

{% tab title="Description" %}
| fieldname | type | description |
| :--- | :--- | :--- |
| name | string |  |
| name | string |  |
| sure\_name | string |  |
| birthdate | string |  |
| mobile\_number | string |  |
| address\_street | string |  |
| address\__postal_\_code | string |  |
| address\_country | string |  |
| address\_city | string |  |
|  |  |  |
{% endtab %}
{% endtabs %}
