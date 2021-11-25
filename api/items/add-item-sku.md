# add item sku

{% swagger method="post" path="/v1/items/sku/:items_id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="items_id" required="true" %}

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

### Json Body

{% tabs %}
{% tab title="json body" %}
```javascript
[
    {
        id        : "120938120938",
        color        : "Blue",
        size        : "L",
        price        : 15.0,
        vendorProductSKU: "SKU-981-B"
    },
    {
        color        : "Red",
        size        : "L",
        price        : 15.0,
        vendorProductSKU: "SKU-981-R",
    },
    {
        color        : "Green",
        vendorProductSKU: "SKU-981-G",
    }
]
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request POST 'http://localhost:8080/v1/items/sku/61976959e9d1a4dd7ded13f0' \
--header 'Authorization: xxxxxx' \
--header 'Content-Type: application/json' \
--data-raw '[
    {
        id        : "120938120938",
        color        : "Blue",
        size        : "L",
        price        : 15.0,
        vendorProductSKU: "SKU-981-B"
    },
    {
        color        : "Red",
        size        : "L",
        price        : 15.0,
        vendorProductSKU: "SKU-981-R",
    },
    {
        color        : "Green",
        vendorProductSKU: "SKU-981-G",
    }
]'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
