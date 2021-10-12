# Update Items

{% swagger method="put" path="/v1/items" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="items_id" %}
item id for update
{% endswagger-parameter %}

{% swagger-parameter in="body" name="body" type="object" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
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
{% endswagger-response %}

{% swagger-response status="400: Bad Request" description="" %}
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
{% endswagger-response %}
{% endswagger %}

### Body Request

{% tabs %}
{% tab title="json body" %}
```javascript
{
    "house_room": "bathroom",
    "product_category": "hair",
    "product_sub_category": "shampoo",
    "price": 10.5,
    "mico_score.co2": 60,
    "mico_score.water": 10,
    "mico_score.non_recyle": true,
    "mico_values.one_hundred_percent_natureal": true,
    "mico_values.plastic_free": true,
    "mico_values.is_recycling": true,
    "mico_values.palm_oil_free": false,
    "mico_values.vegan": true,
    "mico_values.is_co2_exist": false,
    "product_overview": "",
    "buying_details.country_origin": "xx",
    "buying_details.delivery_time": "xx",
    "buying_details.delivery_packaging": "xx"
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
curl --location --request PUT 'http://localhost:8080/v1/items/61650a7600e3fa3ad0176ca2' \
--header 'Authorization: xxxxxxxx' \
--header 'Content-Type: application/json' \
--data-raw '{
    "house_room": "bathroom",
    "product_category": "hair",
    "product_sub_category": "shampoo",
    "price": 10.5,
    "mico_score.co2": 60,
    "mico_score.water": 10,
    "mico_score.non_recyle": true,
    "mico_values.one_hundred_percent_natureal": true,
    "mico_values.plastic_free": true,
    "mico_values.is_recycling": true,
    "mico_values.palm_oil_free": false,
    "mico_values.vegan": true,
    "mico_values.is_co2_exist": false,
    "product_overview": "",
    "buying_details.country_origin": "xx",
    "buying_details.delivery_time": "xx",
    "buying_details.delivery_packaging": "xx"
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
