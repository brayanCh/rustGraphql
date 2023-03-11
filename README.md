<h1>Graphql api made in rust with actix</h1>

<p>
    The main objective of this project is to show it in my portfolio, other could be getting better at rust and backend in general, it also could be used as an example on how rust could be used in more production oriented projects instead of being a c++ replacement, it could be used in the same enviroment as java, c#, php, etc.
</p>

<h3>DB structure</h3>

<h4> user </h4>

<ul>
    <li>ID : string</li>
    <li>name : string</li>
    <li>email : string</li>
    <li>cellnumber : string</li>
    <li>profilePicUrl : string</li>
    <li>planType : string</li>
    <li>registerDay: int || unixTimeStamp</li>
    <li>lastPaymentDay: int || unixTimeStamp</li>
    <li>hasCancelledTheService: boolean</li>
</ul>

<h4> employee </h4>

<ul>
    <li>ID : string</li>
    <li>name : string</li>
    <li>email : string</li>
    <li>cellnumber : string</li>
    <li>profilePicUrl : string</li>
    <li>monthlyWage : int</li>
    <li>lastWageDay: int || unixTimeStamp</li>
    <li>dayBeginningVacations: int || unixTimeStamp</li>
    <li>lengthVacations: int || unixTimeStamp</li>
    <li>registerDay: int || unixTimeStamp</li>
    <li>isStillEmployed: boolean</li>
    <li>isInVacation: boolean</li>
    <li>isAdmin: boolean</li>
</ul>

<h4> equipment </h4>

<ul>
    <li>ID : string</li>
    <li>name : string</li>
    <li>price : int</li>
    <li>purchaseDay: int || unixTimeStamp</li>
    <li>expectedMaintenanceDate: int || unixTimeStamp</li>
    <li>hasBeenThrowAway: boolean</li>
    <li>isInMaintenance : boolean</li>
</ul>

