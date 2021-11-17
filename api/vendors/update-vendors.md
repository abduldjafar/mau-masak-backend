# Update Vendors

{% swagger method="put" path="/v1/vendors/:vendor_id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="vendor_id" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}
{% endswagger %}

### Body Requests

{% tabs %}
{% tab title="Json Body" %}
```
// can choose one or more for update
{
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
    "vendor_owner_name": "",
    "vendor_owner_phone_numbers": "",
    "vendor_owner_representative_name": "",
    "vendor_owner_email": "",
    "order_channel": "",
    "billing_conditions": "",
    "payment_terms": "",
    "currency": ""
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request PUT 'http://localhost:8080/v1/vendors/61525154347c025c4cdc29c0' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNjQwZTkwMjI5NThkNGNkYmQ5MWVlMSIsImVtYWlsIjoicnV0cmV0dGFzb2ljcnUtNzEwM0B5b3BtYWlsLmNvbSIsImRhdGF0eXBlIjoiYyIsImV4cCI6MTYzNDA5NDEyOH0.gDe3UY1veX_m_3EgkeLN4F5fkGVgI-WUyGKETltnF2I' \
--header 'Content-Type: application/json' \
--data-raw '{
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
    "vendor_owner_name": "",
    "vendor_owner_phone_numbers": "",
    "vendor_owner_representative_name": "",
    "vendor_owner_email": "",
    "order_channel": "",
    "billing_conditions": "",
    "payment_terms": "",
    "currency": ""
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
