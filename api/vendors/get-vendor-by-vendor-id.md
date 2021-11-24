# get vendor by vendor id

{% swagger method="get" path="/v1/vendors/:vendor_id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="vendor_id" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "_id": "619e3a03cae42e8ff93dd9bb",
            "vendor_name": "SQ",
            "registered_name": "qSq",
            "country_origin": "",
            "vat_number": "`112",
            "businnes_models": "franchise",
            "certification": "qsSsd",
            "specialization_product": "vegetables",
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
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
