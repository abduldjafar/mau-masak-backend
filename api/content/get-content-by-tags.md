# get content by tags

{% swagger method="get" path="/v1/users/pedia/tags" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" required="true" name="tags_name" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61ae32768488c5e571b51dd9",
                "user_id": "61a9dabb8f2d1be4629eb7cb",
                "user_name": "",
                "user_picture": "",
                "user_type": "user",
                "picture": "https://storage.googleapis.com/mico_project///paint_in_water_background-wallpaper-1920x1080.jpg",
                "created_at": "2021-12-06T15:55:34.429Z",
                "body_content": {
                    "time": 1638843698793,
                    "blocks": [
                        {
                            "id": "PPIhy0VwTu",
                            "type": "paragraph",
                            "data": {
                                "text": "Hello There! I'm Here~<br>",
                                "level": 0,
                                "file": {
                                    "url": ""
                                },
                                "caption": "",
                                "withBorder": false,
                                "stretched": false,
                                "withBackground": false,
                                "style": "",
                                "items": null
                            }
                        }
                    ],
                    "version": "2.22.2"
                },
                "title": "This is Testing Article",
                "city": "",
                "country": "Albania",
                "tags": [
                    "Testing Tags"
                ],
                "sub_tags": [
                    "Testing Sub Tags"
                ],
                "sub_sub_tags": [
                    "Sub Sub Tags Nameeeee"
                ]
            },
            {
                "_id": "61aebec5a12433553dc8dcb7",
                "user_id": "618cda10d21a63750ee97699",
                "user_name": "",
                "user_picture": "",
                "user_type": "admin",
                "picture": "https://storage.googleapis.com/mico_project///2020-11-26.png",
                "created_at": "2021-12-07T01:54:13.071Z",
                "body_content": {
                    "time": 1638842052662,
                    "blocks": [
                        {
                            "id": "EH1khaZC9X",
                            "type": "paragraph",
                            "data": {
                                "text": "Hello There!<br>",
                                "level": 0,
                                "file": {
                                    "url": ""
                                },
                                "caption": "",
                                "withBorder": false,
                                "stretched": false,
                                "withBackground": false,
                                "style": "",
                                "items": null
                            }
                        }
                    ],
                    "version": "2.22.2"
                },
                "title": "Admin Articles!",
                "city": "",
                "country": "Belarus",
                "tags": [
                    "Testing Tags"
                ],
                "sub_tags": [
                    "Testing Sub Tags"
                ],
                "sub_sub_tags": [
                    "Sub Sub Tags Nameeeee"
                ]
            },
            {
                "_id": "61af08cc915a984042c6781c",
                "user_id": "618cda10d21a63750ee97699",
                "user_name": "",
                "user_picture": "",
                "user_type": "admin",
                "picture": "https://storage.googleapis.com/mico_project///Blue-Camouflage-Wallpaper.jpg",
                "created_at": "2021-12-07T07:10:04.83Z",
                "body_content": {
                    "time": 1638861002510,
                    "blocks": [
                        {
                            "id": "cXBVWpMQDV",
                            "type": "paragraph",
                            "data": {
                                "text": "Hello There!<br>",
                                "level": 0,
                                "file": {
                                    "url": ""
                                },
                                "caption": "",
                                "withBorder": false,
                                "stretched": false,
                                "withBackground": false,
                                "style": "",
                                "items": null
                            }
                        }
                    ],
                    "version": "2.22.2"
                },
                "title": "Testing 4",
                "city": "",
                "country": "Global",
                "tags": [
                    "tags-1 update",
                    "Testing Tags"
                ],
                "sub_tags": [
                    "sub-tags-1-1",
                    "Testing Sub Tags"
                ],
                "sub_sub_tags": [
                    "sub-sub-tags-1-1",
                    "Testin Sub sub Tags"
                ]
            },
            {
                "_id": "61af08ef915a984042c6781d",
                "user_id": "618cda10d21a63750ee97699",
                "user_name": "",
                "user_picture": "",
                "user_type": "admin",
                "picture": "https://storage.googleapis.com/mico_project///i_can_hide___-wallpaper-1920x1080.jpg",
                "created_at": "2021-12-07T07:10:39.889Z",
                "body_content": {
                    "time": 1638861037020,
                    "blocks": [
                        {
                            "id": "56aTox6bfV",
                            "type": "paragraph",
                            "data": {
                                "text": "This is Testing 5 for checking listing on content list<br>",
                                "level": 0,
                                "file": {
                                    "url": ""
                                },
                                "caption": "",
                                "withBorder": false,
                                "stretched": false,
                                "withBackground": false,
                                "style": "",
                                "items": null
                            }
                        }
                    ],
                    "version": "2.22.2"
                },
                "title": "Testing 5",
                "city": "",
                "country": "Global",
                "tags": [
                    "Testing Tags Here",
                    "Testing Tags",
                    "tags-1 update"
                ],
                "sub_tags": [
                    "Test Sub Tags",
                    "Testing Sub Tags",
                    "sub-tags-1-1"
                ],
                "sub_sub_tags": [
                    "Test Sub Sub Tags",
                    "Sub Sub Tags Nameeeee",
                    "sub-sub-tags-1-1"
                ]
            },
            {
                "_id": "61af3b2838f36fd3e477812d",
                "user_id": "61a9dabb8f2d1be4629eb7cb",
                "user_name": "Joe Phang Rahmansyah",
                "user_picture": "",
                "user_type": "user",
                "picture": "https://storage.googleapis.com/mico_project///xperia_z_ultra-wallpaper-1920x1080.jpg",
                "created_at": "2021-12-07T10:44:56.915Z",
                "body_content": {
                    "time": 1638873894473,
                    "blocks": [
                        {
                            "id": "W0gNRhDo1X",
                            "type": "paragraph",
                            "data": {
                                "text": "Hello There! I'm Here~<br>",
                                "level": 0,
                                "file": {
                                    "url": ""
                                },
                                "caption": "",
                                "withBorder": false,
                                "stretched": false,
                                "withBackground": false,
                                "style": "",
                                "items": null
                            }
                        }
                    ],
                    "version": "2.22.2"
                },
                "title": "Testing Joe's Article",
                "city": "",
                "country": "Global",
                "tags": [
                    "Testing Tags",
                    "tags-1 update"
                ],
                "sub_tags": [
                    "Testing Sub Tags",
                    "sub-tags-1-1"
                ],
                "sub_sub_tags": [
                    "Sub Sub Tags Nameeeee",
                    "sub-sub-tags-1-1"
                ]
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
