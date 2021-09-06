---
description: Api about talents
---

# Talents

{% api-method method="get" host="http://" path="backend-services-prdd5cupna-as.a.run.app/talents" %}
{% api-method-summary %}
Get Talents Datas
{% endapi-method-summary %}

{% api-method-description %}

{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-path-parameters %}
{% api-method-parameter name="user\_id" type="string" required=false %}
user\_id users
{% endapi-method-parameter %}
{% endapi-method-path-parameters %}

{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=false %}
token for access the api
{% endapi-method-parameter %}
{% endapi-method-headers %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```
{
  "responses": {
    "code": 200,
    "data": {
      "_id": "6116097f4f42047d91970c80",
      "address": "Lembah Furia",
      "age": 0,
      "banner": null,
      "bodysize": {
        "bodytype": {
          "id": "6110e163e813e55ee9ab7608",
          "value": "Muscular"
        },
        "bust": 123,
        "eyescolor": {
          "id": "6110e109e813e55ee9aae207",
          "value": "Violet"
        },
        "haircolor": {
          "id": "6110e05de813e55ee9a9c2a2",
          "value": "Muscular"
        },
        "hairtype": {
          "id": "61160b827e74d5f6958e98b8",
          "value": "Bald"
        },
        "height": 135,
        "hips": 90,
        "shoessize": 42,
        "waist": 11
      },
      "description": "",
      "email": "gadoveiwoiju-1564@yopmail.com",
      "ethnicity": {
        "id": "6110de12e813e55ee9a668c5",
        "value": "Oceania"
      },
      "gender": {
        "id": "6110d7b2e813e55ee99dce35",
        "value": "Transgender"
      },
      "keywords": [
        "Hot",
        "Nature"
      ],
      "languages": [
        {
          "id": "61160d447e74d5f69590cba5",
          "value": "English"
        }
      ],
      "name": "Abdul Haris Djafar",
      "password": "",
      "productiontype": [
        {
          "id": "61160d447e74d5f69590cba5",
          "value": "English"
        },
        {
          "id": "6110db1fe813e55ee9a23b97",
          "value": "Extra"
        }
      ],
      "ratings_count": {
        "communication": 0,
        "professionality": 0,
        "innerbeauty": 0,
        "modeling": 0,
        "selfconfidence": 0,
        "independent": 0,
        "attractiveappearance": 0,
        "teamwork": 0,
        "acting": 0,
        "timemanagement": 0
      },
      "skills": [
        {
          "id": "61160d447e74d5f69590cba5",
          "value": "English"
        },
        {
          "id": "6110df8fe813e55ee9a89fbf",
          "value": "Coordination"
        }
      ],
      "username": ""
    },
    "isError": false,
    "message": "success"
  }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=400 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```
{
  "responses": {
    "code": 400,
    "data": {},
    "isError": true,
    "message": "the message will adjust to the server"
  }
}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}



