# Add Item Into Cart

{% api-method method="post" host="https://api.cakes.com" path="/v1/items/cart" %}
{% api-method-summary %}
Get Cakes
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
        "data": null,
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

### Body Request

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
  "items_id":"615d7aee24605684a912f493",
	"items_colour":"red",
	"items_size":"xl",
	"count":1
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request POST 'http://localhost:8080/v1/items/cart' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzM2Njk0MTd9.vpjZ6hS7hu_R_9SGmanDDmem2Y9VkodHsF6yJtDKPL4' \
--header 'Content-Type: application/json' \
--data-raw '{
    "items_id":"615d7aee24605684a912f493",
	"items_colour":"red",
	"items_size":"xl",
	"count":1
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

