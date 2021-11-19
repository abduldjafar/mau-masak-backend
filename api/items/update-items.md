# Update Items

{% swagger method="put" path="/v1/items/:items_id" baseUrl="" summary="" %}
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
// can choose one or more from those field
{
    "vendor_id": "",
    "vendor_description": "",
    "brand_name": "",
    "house_room": "",
    "product_category": "",
    "product_sub_category": "",
    "product_name": "",
    "price":0,
    "stripe_price_id": "",
    "product_stripe_id": "",
    "units":0,
    "mico_values.vegan.active":false,
    "mico_values.vegan.value":"",
    "mico_values.plastic_free.active":false,
    "mico_values.plastic_free.value":"",
    "mico_values.organic.active":false,
    "mico_values.organic.value":false,
    "mico_values.glycerin_free.active":false,
    "mico_values.glycerin_free.value":false,
    "mico_values.palm_oil_free.active":false,
    "mico_values.palm_oil_free.value":"",
    "mico_values.local.active":false,
    "mico_values.local.value":"",
    "mico_values.compostable.active":false,
    "mico_values.compostable.value":"",
    "mico_values.biodegradable.active":false,
    "mico_values.biodegradable.value":"",
    "mico_values.recyclable.active":false,
    "mico_values.recyclable.value":"",
    "mico_values.ethically_produced.active":false,
    "mico_values.ethically_produced.value":"",
    "product_description": "",
    "ingredients_materials": "",
    "howto_handle": "",
    "manufacturing_address": "",
    "shipping_from": "",
    "shipping_method": "",
    "shipping_courier": "",
    "shipping_cost": 0,
    "delivery_time":"",
    "star":0,
    "type_of_packaging": "",
    "kg_of_packaging": 0,
    "minimum_order_quantity":0,
    "published": false,
    "average_yearly_consumption": 0
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
