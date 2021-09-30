# User Update Address

{% api-method method="put" host="https://api.cakes.com" path="/v1/cakes/:id" %}
{% api-method-summary %}
Update Address
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-body-parameters %}
{% api-method-parameter name="body" type="object" required=true %}

{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```
{    "name": "Cake's name",    "recipe": "Cake's recipe name",    "cake": "Binary cake"}
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

### Request Body

{% tabs %}
{% tab title="Json Body" %}
```go
{
    "_id": "6155290af8e7be3a693ffaa7",
    "recipent_name": "aasoesoi",
    "country": "asoi",
    "phone_number": "asoi",
    "address_line1": "xxxx",
    "address_line2": "asoi",
    "city": "asoi",
    "state": "asoi",
    "postal_code": "asoi"
}
```
{% endtab %}

{% tab title="Description" %}
| field | type | description |
| :--- | :--- | :--- |
| \_id | string | address id |

 
{% endtab %}
{% endtabs %}

