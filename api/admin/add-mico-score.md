# add mico score

{% swagger method="post" path="/v1/items/mico_scores" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}
{% endswagger %}

### Json Body

{% tabs %}
{% tab title="Body" %}
```javascript
{
    "type": "ASOI",
    "size": "1.5L",
    "weight": 1,
    "co2_kg": 2,
    "water_liters": 2,
    "earth_waste_in_years": 1
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
