# Get House Part

{% swagger baseUrl="https://api.cakes.com" path="/v1/house_part" method="get" summary="Get House Part" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" type="string" %}
Authentication token to track down who is emptying our stocks.
{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="integer" %}
The API will do its best to find a cake matching the provided recipe
{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="integer" %}
Whether the cake should be gluten-free or not.
{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
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
{% endswagger-response %}

{% swagger-response status="404" description="Could not find a cake matching this query." %}
```
{    "message": "Ain't no cake like that."}
```
{% endswagger-response %}
{% endswagger %}

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
