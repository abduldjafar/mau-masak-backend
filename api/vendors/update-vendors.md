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
{
    "brand_or_vendor": "manga asiksssss",
    "HQ_country": "",
    "business_model": "",
    "phone_number": "",
    "email": "manga@asik.com",
    "address": "",
    "owner_name": "",
    "representative_name": "",
    "official_register_name": "",
    "vat_number": "",
    "bank_account": "",
    "certified_b_corporation": "",
    "category_or_products": ""
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
    "brand_or_vendor": "manga asiksssss",
    "HQ_country": "",
    "business_model": "",
    "phone_number": "",
    "email": "manga@asik.com",
    "address": "",
    "owner_name": "",
    "representative_name": "",
    "official_register_name": "",
    "vat_number": "",
    "bank_account": "",
    "certified_b_corporation": "",
    "category_or_products": ""
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
