# get persona questions

{% swagger method="get" path="/v1/users/persona_questions" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="limit" type="int" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "question_number": 1,
                "questin_string": "I go by?",
                "answer": {
                    "a": "Mr.",
                    "b": "Mrs.",
                    "c": "Mrx.",
                    "d": "",
                    "e": "",
                    "f": ""
                }
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
