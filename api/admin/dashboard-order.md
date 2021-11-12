# dashboard order

{% swagger method="get" path="/v1/admin/dasboard_order" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "617f3cc395a6b71e01eb2031",
                "user_id": "61640e9022958d4cdbd91ee1",
                "items_in_cart": [
                    {
                        "_id": "617f330960d515c11856b62e",
                        "checkout_id": "617f330960d515c11856b62d",
                        "user_id": "61640e9022958d4cdbd91ee1",
                        "items_id": "617e4c43117bd704d9e6051b",
                        "items_colour": "red",
                        "items_size": "xl",
                        "count": 1,
                        "price_idin_stripe": "price_1JpsT6F8sq9RvyoVynrMgfui",
                        "user_stripe_id": "cus_KO7lOem6Z4A04p",
                        "product_name": "",
                        "price": 0,
                        "mico_score": {
                            "co2": 0,
                            "water": 0,
                            "non_recyle": false
                        },
                        "selected": true,
                        "paid": false,
                        "deleted": false,
                        "activate": true,
                        "created_at": "2021-11-01T00:21:29.37Z"
                    }
                ],
                "product_count": 1,
                "paid": true,
                "total_paid": 0,
                "product_paid": 0,
                "shipping_paid": 0,
                "url_checkout": "https://checkout.stripe.com/pay/cs_test_b1tk5dfyxJ8qCWM2O9PA3Q4YkNwBPNIkHVEnfMPaUvP3OHd9NqDbWnK4d0#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
                "created_at": "2021-11-01T01:02:59.988Z",
                "paid_date": "2021-11-01T01:03:24.868Z",
                "shipping_methods": "J&T",
                "shipping_address": "",
                "payment_gateway_id": "pi_3JqouLF8sq9RvyoV0hvGikaY"
            },
            {
                "_id": "617f3d40c48cc07ef537a660",
                "user_id": "61640e9022958d4cdbd91ee1",
                "items_in_cart": [
                    {
                        "_id": "617f330960d515c11856b62e",
                        "checkout_id": "617f330960d515c11856b62d",
                        "user_id": "61640e9022958d4cdbd91ee1",
                        "items_id": "617e4c43117bd704d9e6051b",
                        "items_colour": "red",
                        "items_size": "xl",
                        "count": 1,
                        "price_idin_stripe": "price_1JpsT6F8sq9RvyoVynrMgfui",
                        "user_stripe_id": "cus_KO7lOem6Z4A04p",
                        "product_name": "",
                        "price": 0,
                        "mico_score": {
                            "co2": 0,
                            "water": 0,
                            "non_recyle": false
                        },
                        "selected": true,
                        "paid": false,
                        "deleted": false,
                        "activate": true,
                        "created_at": "2021-11-01T00:21:29.37Z"
                    }
                ],
                "product_count": 1,
                "paid": true,
                "total_paid": 0,
                "product_paid": 0,
                "shipping_paid": 0,
                "url_checkout": "https://checkout.stripe.com/pay/cs_test_b1Q0yBdKYYX0vE7gA9DTcelkZYDAld9B3VqauifMzuFpNITlnshvRPpnsg#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
                "created_at": "2021-11-01T01:05:04.854Z",
                "paid_date": "2021-11-01T01:05:42.357Z",
                "shipping_methods": "J&T",
                "shipping_address": "",
                "payment_gateway_id": "pi_3JqowMF8sq9RvyoV09sSdORw"
            },
            {
                "_id": "617f3eb9ef5700daf9051366",
                "user_id": "61640e9022958d4cdbd91ee1",
                "items_in_cart": [
                    {
                        "_id": "617f3de1ef5700daf9051365",
                        "checkout_id": "617f3de1ef5700daf9051364",
                        "user_id": "61640e9022958d4cdbd91ee1",
                        "items_id": "617e4c43117bd704d9e6051e",
                        "items_colour": "red",
                        "items_size": "xl",
                        "count": 1,
                        "price_idin_stripe": "price_1JqHaoF8sq9RvyoVcEw5aSwH",
                        "user_stripe_id": "cus_KO7lOem6Z4A04p",
                        "product_name": "",
                        "price": 0,
                        "mico_score": {
                            "co2": 0,
                            "water": 0,
                            "non_recyle": false
                        },
                        "selected": true,
                        "paid": false,
                        "deleted": false,
                        "activate": true,
                        "created_at": "2021-11-01T01:07:45.473Z"
                    }
                ],
                "product_count": 1,
                "paid": true,
                "total_paid": 0,
                "product_paid": 0,
                "shipping_paid": 0,
                "url_checkout": "https://checkout.stripe.com/pay/cs_test_b1hrPuzPtQ64uQWdih8j8hoRQbjIDyspAbO3QcVgGv1YsA4nNt15N3FvWX#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
                "created_at": "2021-11-01T01:11:21.291Z",
                "paid_date": "2021-11-01T01:12:51.795Z",
                "shipping_methods": "J&T",
                "shipping_address": "",
                "payment_gateway_id": "pi_3Jqp3VF8sq9RvyoV1kdf1GFA"
            },
            {
                "_id": "617f431e36d741c1fcf8f56d",
                "user_id": "61640e9022958d4cdbd91ee1",
                "items_in_cart": [
                    {
                        "_id": "617f430236d741c1fcf8f56c",
                        "checkout_id": "617f430236d741c1fcf8f56b",
                        "user_id": "61640e9022958d4cdbd91ee1",
                        "items_id": "617e4c43117bd704d9e6051b",
                        "items_colour": "",
                        "items_size": "",
                        "count": 1,
                        "price_idin_stripe": "price_1JpsT6F8sq9RvyoVynrMgfui",
                        "user_stripe_id": "cus_KO7lOem6Z4A04p",
                        "product_name": "",
                        "price": 0,
                        "mico_score": {
                            "co2": 0,
                            "water": 0,
                            "non_recyle": false
                        },
                        "selected": true,
                        "paid": false,
                        "deleted": false,
                        "activate": true,
                        "created_at": "2021-11-01T01:29:38.288Z"
                    }
                ],
                "product_count": 1,
                "paid": true,
                "total_paid": 0,
                "product_paid": 0,
                "shipping_paid": 0,
                "url_checkout": "https://checkout.stripe.com/pay/cs_test_b1FuryOUZ1gDOQMg7RtcGxeKxn0kS213e4XwNJ8G68ep7GGK5386fX3Vu1#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
                "created_at": "2021-11-01T01:30:06.03Z",
                "paid_date": "2021-11-01T01:30:24.873Z",
                "shipping_methods": "J&T",
                "shipping_address": "",
                "payment_gateway_id": "pi_3JqpLdF8sq9RvyoV2fx2FDBp"
            },
            {
                "_id": "617f438836d741c1fcf8f572",
                "user_id": "61640e9022958d4cdbd91ee1",
                "items_in_cart": [
                    {
                        "_id": "617f435236d741c1fcf8f56f",
                        "checkout_id": "617f435236d741c1fcf8f56e",
                        "user_id": "61640e9022958d4cdbd91ee1",
                        "items_id": "617e4c43117bd704d9e6051b",
                        "items_colour": "",
                        "items_size": "",
                        "count": 1,
                        "price_idin_stripe": "price_1JpsT6F8sq9RvyoVynrMgfui",
                        "user_stripe_id": "cus_KO7lOem6Z4A04p",
                        "product_name": "",
                        "price": 0,
                        "mico_score": {
                            "co2": 0,
                            "water": 0,
                            "non_recyle": false
                        },
                        "selected": true,
                        "paid": false,
                        "deleted": false,
                        "activate": true,
                        "created_at": "2021-11-01T01:30:58.322Z"
                    },
                    {
                        "_id": "617f436a36d741c1fcf8f571",
                        "checkout_id": "617f436936d741c1fcf8f570",
                        "user_id": "61640e9022958d4cdbd91ee1",
                        "items_id": "617e4c43117bd704d9e60520",
                        "items_colour": "",
                        "items_size": "",
                        "count": 1,
                        "price_idin_stripe": "price_1JqHrwF8sq9RvyoVWpsWmTEx",
                        "user_stripe_id": "cus_KO7lOem6Z4A04p",
                        "product_name": "",
                        "price": 0,
                        "mico_score": {
                            "co2": 0,
                            "water": 0,
                            "non_recyle": false
                        },
                        "selected": true,
                        "paid": false,
                        "deleted": false,
                        "activate": true,
                        "created_at": "2021-11-01T01:31:22.161Z"
                    }
                ],
                "product_count": 2,
                "paid": true,
                "total_paid": 0,
                "product_paid": 0,
                "shipping_paid": 0,
                "url_checkout": "https://checkout.stripe.com/pay/cs_test_b1nDFFMMVrGNDwl6SXIzvqR2bPbj9v9my3HqRn1cnKiSMpV2cv1NFvRo1Q#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
                "created_at": "2021-11-01T01:31:52.729Z",
                "paid_date": "2021-11-01T01:32:24.452Z",
                "shipping_methods": "J&T",
                "shipping_address": "",
                "payment_gateway_id": "pi_3JqpNMF8sq9RvyoV2JJes4Nm"
            }
        ],
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
        "data":null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}
