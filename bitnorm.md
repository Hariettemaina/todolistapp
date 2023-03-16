## Util Module
#### index.js
> A module that exports an object containing "pushNotifications" and its value is the result of requiring another module located at "./push_notification".
> This code is creating a module that can be included in other parts of an application and provides access to the functionality exported by the "./push_notification" module.
#### push_notification.js
> Imports the "onesignal-node" module, which is a Node.js client library for the OneSignal API, which
> is then assigned to a constant called "OneSignal", which can be used to interact with the OneSignal > API in the rest of the code. 

> The function takes in a data object as an argument, which contains information about the notification to be sent, such as the content, title, and the userId of the recipient.

>The OneSignal API requires an API key and an App ID to be able to send notifications. In this code, these credentials are passed to the OneSignal client object, which is then used to send the notification using the "createNotification" method.

> The function uses a try-catch block to handle any errors that might occur while sending the notification. If an error occurs, it is logged to the console with the status code and body of the error response.
## Users
#### user.js
> This code defines a Mongoose schema for a user model.
>The schema has properties for the user's email address, display name, bnTokens, bookmarks, earned and reputation points, bio, work, education, honors, courses, projects, skills, and languages. There are also fields for the user's gender, address, location, hash (presumably a password hash), password reset code, type, and social media profiles. Other properties include reputation, blocked status, portfolio, website, profile picture, and cover picture.

>The schema includes data types and validation rules for each property, such as type: String, type: Number, type: Boolean, type: Date, etc. Additionally, some fields are set as required and have default values.

>The schema references other models, such as 'Social' and 'Image', through the socials and profile_pic/cover_pic properties.
> its also check if the passwords entered match