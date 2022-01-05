# get house part full children

{% swagger method="get" path="/v1/house_part/full" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61a0c41b52982f63ce64d7d1",
                "house_part_category": [
                    {
                        "_id": "61a656867a283cf6e256b037",
                        "banner_text": "",
                        "house_part_id": "61a0c41b52982f63ce64d7d1",
                        "house_part_subcategory": [
                            {
                                "_id": "61a657de7a283cf6e256b038",
                                "category_id": "61a656867a283cf6e256b037",
                                "house_part_id": "",
                                "name": "Test Sub Category",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/61a657de7a283cf6e256b038/gitlab_workflow.png"
                            },
                            {
                                "_id": "61a7370dc408c61591a040c6",
                                "category_id": "61a656867a283cf6e256b037",
                                "house_part_id": "",
                                "name": "Test",
                                "picture": ""
                            }
                        ],
                        "name": "Test Category",
                        "picture": "https://storage.googleapis.com/mico_project/category_item/61a656867a283cf6e256b037/gitlab_workflow.png"
                    }
                ],
                "house_part_name": "tes",
                "picture": "https://storage.googleapis.com/mico_project/house_path/61a0c41b52982f63ce64d7d1/beautiful_mountain_landscape_dolomites_italy-wallpaper-1600x900.jpg"
            },
            {
                "_id": "617bc5b36039d894317b2b72",
                "house_part_category": [
                    {
                        "_id": "617bc736b6028c6baeb8bad0",
                        "banner_text": "Eco-friendly products to clean all corners of the house and protect your health in one!",
                        "house_part_id": "617bc5b36039d894317b2b72",
                        "house_part_subcategory": [
                            {
                                "_id": "617d46f95fd4974ba1e74e3a",
                                "category_id": "617bc736b6028c6baeb8bad0",
                                "house_part_id": "617bc5b36039d894317b2b72",
                                "name": "sub-category-1",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/617d46f95fd4974ba1e74e3a/Allround%20cleaner.jpeg"
                            }
                        ],
                        "name": "Allround Cleaner",
                        "picture": "https://storage.googleapis.com/mico_project/category_item/617bc736b6028c6baeb8bad0/Allround%20cleaner.jpeg"
                    }
                ],
                "house_part_name": "Cleaning",
                "picture": "https://storage.googleapis.com/mico_project/house_path/617bc5b36039d894317b2b72/cleaning.jpeg"
            },
            {
                "_id": "617bc5d76039d894317b2b73",
                "house_part_category": [
                    {
                        "_id": "617bc767b6028c6baeb8bad1",
                        "banner_text": "Diverting old coffee to serve your daily new coffee! ",
                        "house_part_id": "617bc5d76039d894317b2b73",
                        "house_part_subcategory": [
                            {
                                "_id": "617d49bd5fd4974ba1e74e3e",
                                "category_id": "617bc767b6028c6baeb8bad1",
                                "house_part_id": "617bc5d76039d894317b2b73",
                                "name": "sub-category-2",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/617d49bd5fd4974ba1e74e3e/coffee%20travel%20cup2.jpeg"
                            }
                        ],
                        "name": "Coffee Travel Cups",
                        "picture": "https://storage.googleapis.com/mico_project/category_item/617bc767b6028c6baeb8bad1/coffee%20travel%20cup2.jpeg"
                    },
                    {
                        "_id": "617bc79ab6028c6baeb8bad2",
                        "banner_text": "Reusable food containers can save resources all year round!",
                        "house_part_id": "617bc5d76039d894317b2b73",
                        "house_part_subcategory": [
                            {
                                "_id": "617d49d05fd4974ba1e74e3f",
                                "category_id": "617bc79ab6028c6baeb8bad2",
                                "house_part_id": "617bc5d76039d894317b2b73",
                                "name": "sub-category-3",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/617d49d05fd4974ba1e74e3f/lunchbox.jpeg"
                            }
                        ],
                        "name": "Lunch Boxes",
                        "picture": "https://storage.googleapis.com/mico_project/category_item/617bc79ab6028c6baeb8bad2/lunchbox1.jpeg"
                    }
                ],
                "house_part_name": "On the move",
                "picture": "https://storage.googleapis.com/mico_project/house_path/617bc5d76039d894317b2b73/On%20The%20Move.jpeg"
            },
            {
                "_id": "61a0c3b552982f63ce64d7d0",
                "house_part_category": [
                    {
                        "_id": "61a0df64133d11402455d61b",
                        "banner_text": "",
                        "house_part_id": "61a0c3b552982f63ce64d7d0",
                        "house_part_subcategory": [
                            {
                                "_id": "61a0df93133d11402455d61c",
                                "category_id": "61a0df64133d11402455d61b",
                                "house_part_id": "",
                                "name": "New Sub Category Here!",
                                "picture": ""
                            }
                        ],
                        "name": "New Category",
                        "picture": ""
                    }
                ],
                "house_part_name": "Tes Create",
                "picture": ""
            },
            {
                "_id": "61aeea50b6d7aaff9c0ce5a9",
                "house_part_category": [
                    {
                        "_id": "61aeea5ab6d7aaff9c0ce5aa",
                        "banner_text": "Testing Banner Here~",
                        "house_part_id": "61aeea50b6d7aaff9c0ce5a9",
                        "house_part_subcategory": [
                            {
                                "_id": "61aeecb2b6d7aaff9c0ce5b4",
                                "category_id": "61aeea5ab6d7aaff9c0ce5aa",
                                "house_part_id": "61aeea50b6d7aaff9c0ce5a9",
                                "name": "Sub Category Level 1",
                                "picture": ""
                            },
                            {
                                "_id": "61aef819b6d7aaff9c0ce5b9",
                                "category_id": "61aeea5ab6d7aaff9c0ce5aa",
                                "house_part_id": "61aeea50b6d7aaff9c0ce5a9",
                                "name": "Sub Category Level 2",
                                "picture": ""
                            },
                            {
                                "_id": "61af0605162c2c0bee83dd13",
                                "category_id": "61aeea5ab6d7aaff9c0ce5aa",
                                "house_part_id": "61aeea50b6d7aaff9c0ce5a9",
                                "name": "Sub Category Level 3",
                                "picture": ""
                            },
                            {
                                "_id": "61af0636162c2c0bee83dd16",
                                "category_id": "61aeea5ab6d7aaff9c0ce5aa",
                                "house_part_id": "61aeea50b6d7aaff9c0ce5a9",
                                "name": "Sub Category Level 4",
                                "picture": ""
                            },
                            {
                                "_id": "61af0650162c2c0bee83dd19",
                                "category_id": "61aeea5ab6d7aaff9c0ce5aa",
                                "house_part_id": "61aeea50b6d7aaff9c0ce5a9",
                                "name": "Sub Category Level 5",
                                "picture": ""
                            }
                        ],
                        "name": "Category Level 1",
                        "picture": ""
                    }
                ],
                "house_part_name": "Level 1",
                "picture": ""
            },
            {
                "_id": "617bc57c6039d894317b2b71",
                "house_part_category": [
                    {
                        "_id": "617bc6e7b6028c6baeb8bace",
                        "banner_text": "Besides cutting down your carbon footprint and diminishing plastic usage, solid shampoo bars promote healthier and shinier hair short AND long term!",
                        "house_part_id": "617bc57c6039d894317b2b71",
                        "house_part_subcategory": [
                            {
                                "_id": "617bc8bdb6028c6baeb8bad3",
                                "category_id": "617bc6e7b6028c6baeb8bace",
                                "house_part_id": "617bc57c6039d894317b2b71",
                                "name": "Shampoo 2",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/617bc8bdb6028c6baeb8bad3/Blue-Camouflage-Wallpaper.jpg"
                            },
                            {
                                "_id": "61a0c72f5156a87b5d3a65e1",
                                "category_id": "617bc6e7b6028c6baeb8bace",
                                "house_part_id": "",
                                "name": "Testing",
                                "picture": ""
                            },
                            {
                                "_id": "61a0dd79133d11402455d61a",
                                "category_id": "617bc6e7b6028c6baeb8bace",
                                "house_part_id": "",
                                "name": "Testing Sub Category",
                                "picture": ""
                            }
                        ],
                        "name": "Hair 2",
                        "picture": "https://storage.googleapis.com/mico_project/category_item/617bc6e7b6028c6baeb8bace/beautiful_singapore_reflection-wallpaper-1920x1080.jpg"
                    },
                    {
                        "_id": "617bc701b6028c6baeb8bacf",
                        "banner_text": "Keeping a bright smile and keeping plastic out of landfills has never been easier!",
                        "house_part_id": "617bc57c6039d894317b2b71",
                        "house_part_subcategory": [
                            {
                                "_id": "617bc91eb6028c6baeb8bad4",
                                "category_id": "617bc701b6028c6baeb8bacf",
                                "house_part_id": "617bc57c6039d894317b2b71",
                                "name": "Toothpaste",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/617bc91eb6028c6baeb8bad4/toothpaste%20clear.jpeg"
                            },
                            {
                                "_id": "617bc929b6028c6baeb8bad5",
                                "category_id": "617bc701b6028c6baeb8bacf",
                                "house_part_id": "617bc57c6039d894317b2b71",
                                "name": "Toothbrush",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/617bc929b6028c6baeb8bad5/toothbrush.jpeg"
                            }
                        ],
                        "name": "Dental care",
                        "picture": "https://storage.googleapis.com/mico_project/category_item/617bc701b6028c6baeb8bacf/dental%20care.jpg"
                    }
                ],
                "house_part_name": "Bathroom 2",
                "picture": "https://storage.googleapis.com/mico_project/house_path/617bc57c6039d894317b2b71/background1.jpg"
            },
            {
                "_id": "61a77916514534ec489c73af",
                "house_part_category": [
                    {
                        "_id": "61a77924514534ec489c73b0",
                        "banner_text": "Testing Banner Text",
                        "house_part_id": "61a77916514534ec489c73af",
                        "house_part_subcategory": [
                            {
                                "_id": "61a7792f514534ec489c73b1",
                                "category_id": "61a77924514534ec489c73b0",
                                "house_part_id": "",
                                "name": "New Sub Category Name",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/61a7792f514534ec489c73b1/astrophotography_ocean_night_stars_sky-wallpaper-1920x1080.jpg"
                            },
                            {
                                "_id": "61a77f3fc4df0d078a5519d1",
                                "category_id": "61a77924514534ec489c73b0",
                                "house_part_id": "",
                                "name": "Another New Sub Category",
                                "picture": ""
                            },
                            {
                                "_id": "61a77f8ec4df0d078a5519d3",
                                "category_id": "61a77924514534ec489c73b0",
                                "house_part_id": "61a77916514534ec489c73af",
                                "name": "Fresh New Sub Category Name",
                                "picture": "https://storage.googleapis.com/mico_project/sub_category_item/61a77f8ec4df0d078a5519d3/astrophotography_ocean_night_stars_sky-wallpaper-1920x1080.jpg"
                            }
                        ],
                        "name": "New Category Name",
                        "picture": "https://storage.googleapis.com/mico_project/category_item/61a77924514534ec489c73b0/astrophotography-wallpaper-1920x1080.jpg"
                    }
                ],
                "house_part_name": "New House Part",
                "picture": "https://storage.googleapis.com/mico_project/house_path/61a77916514534ec489c73af/beautiful_mountain_landscape_dolomites_italy-wallpaper-1600x900.jpg"
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
