# Get House Part

{% api-method method="get" host="https://api.cakes.com" path="/v1/house\_part" %}
{% api-method-summary %}
Get House Part
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-query-parameters %}
{% api-method-parameter name="page" type="integer" %}
The API will do its best to find a cake matching the provided recipe
{% endapi-method-parameter %}

{% api-method-parameter name="limit" type="integer" %}
Whether the cake should be gluten-free or not.
{% endapi-method-parameter %}
{% endapi-method-query-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "615a4fcd29ca5f0dfb681089",
                "house_part_name": "bathroom"
            },
            {
                "_id": "615a532376fac0c44a4e6c45",
                "house_part_name": "bathroom"
            },
            {
                "_id": "615a534676fac0c44a4e6c46",
                "house_part_name": "livingroom"
            },
            {
                "_id": "615a534f76fac0c44a4e6c47",
                "house_part_name": "livingroom"
            },
            {
                "_id": "615a5a8b6f16322e92208508",
                "house_part_name": "livingroom"
            }
        ],
        "isError": false,
        "message": "success"
    }
}
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

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request GET 'http://localhost:8080/v1/house_part?page=0&limit=1' \
--header 'Authorization: xxxxxxx'
```
{% endtab %}

{% tab title="JavaScript - Fetch" %}
```javascript
var myHeaders = new Headers();
myHeaders.append("Authorization", "xxxxxxx");

var requestOptions = {
  method: 'GET',
  headers: myHeaders,
  redirect: 'follow'
};

fetch("http://localhost:8080/v1/house_part?page=0&limit=1", requestOptions)
  .then(response => response.text())
  .then(result => console.log(result))
  .catch(error => console.log('error', error));
```
{% endtab %}

{% tab title="JavaScript - jQuery" %}
```javascript
var settings = {
  "url": "http://localhost:8080/v1/house_part?page=0&limit=1",
  "method": "GET",
  "timeout": 0,
  "headers": {
    "Authorization": "xxxxxxx"
  },
};

$.ajax(settings).done(function (response) {
  console.log(response);
});
```
{% endtab %}
{% endtabs %}

