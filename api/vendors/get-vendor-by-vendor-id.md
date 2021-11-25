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
            "_id": "61525154347c025c4cdc29c0",
                "vendor_name": "",
                "registered_name": "",
                "country_origin": "",
                "vat_number": "",
                "businnes_models": "",
                "certification": "",
                "specialization_product": "",
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
                "location": "",
                "bank_name": "",
                "account_number": "",
                "account_holder": "",
                "bank_account_exp_date": "0001-01-01T00:00:00Z",
                "shipping_method": "",
                "shipping_partner": "",
                "shipping_cost_b2b": 0,
                "shipping_cost_b2c": 0,
                "published": false
        },
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
