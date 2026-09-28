use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailJobType {
    NewsletterConfirmation,
    WaitlistConfirmation,
    ContactFormAutoResponse,
    WelcomeEmail,
    Custom(String),
}

impl EmailJobType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NewsletterConfirmation => "newsletter_confirmation",
            Self::WaitlistConfirmation => "waitlist_confirmation",
            Self::ContactFormAutoResponse => "contact_form_auto_response",
            Self::WelcomeEmail => "welcome_email",
            Self::Custom(s) => s,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailJobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl EmailJobStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailEventType {
    Sent,
    Delivered,
    Opened,
    Clicked,
    Bounced,
    Complained,
    Unsubscribed,
}

impl EmailEventType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Sent => "sent",
            Self::Delivered => "delivered",
            Self::Opened => "opened",
            Self::Clicked => "clicked",
            Self::Bounced => "bounced",
            Self::Complained => "complained",
            Self::Unsubscribed => "unsubscribed",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuppressionType {
    Bounce,
    Complaint,
    Unsubscribe,
    Manual,
}

impl SuppressionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bounce => "bounce",
            Self::Complaint => "complaint",
            Self::Unsubscribe => "unsubscribe",
            Self::Manual => "manual",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailJob {
    pub id: Uuid,
    pub job_type: String,
    pub recipient_email: String,
    pub template_name: String,
    pub template_data: serde_json::Value,
    pub status: String,
    pub priority: i32,
    pub attempts: i32,
    pub max_attempts: i32,
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailEvent {
    pub id: Uuid,
    pub email_job_id: Option<Uuid>,
    pub message_id: Option<String>,
    pub event_type: String,
    pub recipient_email: String,
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSuppression {
    pub id: Uuid,
    pub email: String,
    pub suppression_type: String,
    pub reason: Option<String>,
    pub bounce_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAnalytics {
    pub template_name: String,
    pub variant_name: Option<String>,
    pub date: chrono::NaiveDate,
    pub sent_count: i32,
    pub delivered_count: i32,
    pub opened_count: i32,
    pub clicked_count: i32,
    pub bounced_count: i32,
    pub complained_count: i32,
    pub unsubscribed_count: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use serde_json::json;

    fn sample_job() -> EmailJob {
        let ts = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        EmailJob {
            id: Uuid::nil(),
            job_type: EmailJobType::WelcomeEmail.as_str().to_string(),
            recipient_email: "user@example.com".to_string(),
            template_name: "welcome".to_string(),
            template_data: json!({ "name": "Ada" }),
            status: EmailJobStatus::Pending.as_str().to_string(),
            priority: 5,
            attempts: 0,
            max_attempts: 3,
            scheduled_at: ts,
            started_at: None,
            completed_at: None,
            failed_at: None,
            error_message: None,
            created_at: ts,
            updated_at: ts,
        }
    }

    fn sample_event() -> EmailEvent {
        let ts = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        EmailEvent {
            id: Uuid::nil(),
            email_job_id: Some(Uuid::nil()),
            message_id: Some("msg-1".to_string()),
            event_type: EmailEventType::Delivered.as_str().to_string(),
            recipient_email: "user@example.com".to_string(),
            timestamp: ts,
            metadata: json!({ "sg_event_id": "abc" }),
            created_at: ts,
        }
    }

    fn sample_suppression() -> EmailSuppression {
        let ts = Utc.with_ymd_and_hms(2024, 1, 2, 3, 4, 5).unwrap();
        EmailSuppression {
            id: Uuid::nil(),
            email: "user@example.com".to_string(),
            suppression_type: SuppressionType::Bounce.as_str().to_string(),
            reason: Some("hard bounce".to_string()),
            bounce_type: Some("hard".to_string()),
            created_at: ts,
            updated_at: ts,
        }
    }

    fn sample_analytics() -> EmailAnalytics {
        EmailAnalytics {
            template_name: "welcome".to_string(),
            variant_name: Some("a".to_string()),
            date: chrono::NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
            sent_count: 10,
            delivered_count: 9,
            opened_count: 4,
            clicked_count: 2,
            bounced_count: 1,
            complained_count: 0,
            unsubscribed_count: 1,
        }
    }

    #[test]
    fn email_job_type_round_trips_and_matches_as_str() {
        let variants = [
            EmailJobType::NewsletterConfirmation,
            EmailJobType::WaitlistConfirmation,
            EmailJobType::ContactFormAutoResponse,
            EmailJobType::WelcomeEmail,
            EmailJobType::Custom("custom_job".to_string()),
        ];
        for variant in variants {
            let encoded = serde_json::to_string(&variant).unwrap();
            let decoded: EmailJobType = serde_json::from_str(&encoded).unwrap();
            assert_eq!(decoded.as_str(), variant.as_str());
        }
    }

    #[test]
    fn email_job_status_round_trips_and_matches_as_str() {
        let variants = [
            EmailJobStatus::Pending,
            EmailJobStatus::Processing,
            EmailJobStatus::Completed,
            EmailJobStatus::Failed,
            EmailJobStatus::Cancelled,
        ];
        for variant in variants {
            let encoded = serde_json::to_string(&variant).unwrap();
            let decoded: EmailJobStatus = serde_json::from_str(&encoded).unwrap();
            assert_eq!(decoded.as_str(), variant.as_str());
        }
    }

    #[test]
    fn email_event_type_round_trips_and_matches_as_str() {
        let variants = [
            EmailEventType::Sent,
            EmailEventType::Delivered,
            EmailEventType::Opened,
            EmailEventType::Clicked,
            EmailEventType::Bounced,
            EmailEventType::Complained,
            EmailEventType::Unsubscribed,
        ];
        for variant in variants {
            let encoded = serde_json::to_string(&variant).unwrap();
            let decoded: EmailEventType = serde_json::from_str(&encoded).unwrap();
            assert_eq!(decoded.as_str(), variant.as_str());
        }
    }

    #[test]
    fn suppression_type_round_trips_and_matches_as_str() {
        let variants = [
            SuppressionType::Bounce,
            SuppressionType::Complaint,
            SuppressionType::Unsubscribe,
            SuppressionType::Manual,
        ];
        for variant in variants {
            let encoded = serde_json::to_string(&variant).unwrap();
            let decoded: SuppressionType = serde_json::from_str(&encoded).unwrap();
            assert_eq!(decoded.as_str(), variant.as_str());
        }
    }

    #[test]
    fn email_job_round_trips() {
        let job = sample_job();
        let encoded = serde_json::to_string(&job).unwrap();
        let decoded: EmailJob = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.id, job.id);
        assert_eq!(decoded.job_type, job.job_type);
        assert_eq!(decoded.recipient_email, job.recipient_email);
        assert_eq!(decoded.template_name, job.template_name);
        assert_eq!(decoded.template_data, job.template_data);
        assert_eq!(decoded.status, job.status);
        assert_eq!(decoded.priority, job.priority);
        assert_eq!(decoded.attempts, job.attempts);
        assert_eq!(decoded.max_attempts, job.max_attempts);
        assert_eq!(decoded.scheduled_at, job.scheduled_at);
        assert_eq!(decoded.started_at, job.started_at);
        assert_eq!(decoded.completed_at, job.completed_at);
        assert_eq!(decoded.failed_at, job.failed_at);
        assert_eq!(decoded.error_message, job.error_message);
        assert_eq!(decoded.created_at, job.created_at);
        assert_eq!(decoded.updated_at, job.updated_at);
    }

    #[test]
    fn email_job_serializes_expected_field_names() {
        let value = serde_json::to_value(sample_job()).unwrap();
        let obj = value.as_object().unwrap();
        for field in [
            "id",
            "job_type",
            "recipient_email",
            "template_name",
            "template_data",
            "status",
            "priority",
            "attempts",
            "max_attempts",
            "scheduled_at",
            "started_at",
            "completed_at",
            "failed_at",
            "error_message",
            "created_at",
            "updated_at",
        ] {
            assert!(obj.contains_key(field), "missing field: {field}");
        }
        assert_eq!(obj.len(), 16);
    }

    #[test]
    fn email_event_round_trips() {
        let event = sample_event();
        let encoded = serde_json::to_string(&event).unwrap();
        let decoded: EmailEvent = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.id, event.id);
        assert_eq!(decoded.email_job_id, event.email_job_id);
        assert_eq!(decoded.message_id, event.message_id);
        assert_eq!(decoded.event_type, event.event_type);
        assert_eq!(decoded.recipient_email, event.recipient_email);
        assert_eq!(decoded.timestamp, event.timestamp);
        assert_eq!(decoded.metadata, event.metadata);
        assert_eq!(decoded.created_at, event.created_at);
    }

    #[test]
    fn email_event_serializes_expected_field_names() {
        let value = serde_json::to_value(sample_event()).unwrap();
        let obj = value.as_object().unwrap();
        for field in [
            "id",
            "email_job_id",
            "message_id",
            "event_type",
            "recipient_email",
            "timestamp",
            "metadata",
            "created_at",
        ] {
            assert!(obj.contains_key(field), "missing field: {field}");
        }
        assert_eq!(obj.len(), 8);
    }

    #[test]
    fn email_suppression_round_trips() {
        let suppression = sample_suppression();
        let encoded = serde_json::to_string(&suppression).unwrap();
        let decoded: EmailSuppression = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.id, suppression.id);
        assert_eq!(decoded.email, suppression.email);
        assert_eq!(decoded.suppression_type, suppression.suppression_type);
        assert_eq!(decoded.reason, suppression.reason);
        assert_eq!(decoded.bounce_type, suppression.bounce_type);
        assert_eq!(decoded.created_at, suppression.created_at);
        assert_eq!(decoded.updated_at, suppression.updated_at);
    }

    #[test]
    fn email_suppression_serializes_expected_field_names() {
        let value = serde_json::to_value(sample_suppression()).unwrap();
        let obj = value.as_object().unwrap();
        for field in [
            "id",
            "email",
            "suppression_type",
            "reason",
            "bounce_type",
            "created_at",
            "updated_at",
        ] {
            assert!(obj.contains_key(field), "missing field: {field}");
        }
        assert_eq!(obj.len(), 7);
    }

    #[test]
    fn email_analytics_round_trips() {
        let analytics = sample_analytics();
        let encoded = serde_json::to_string(&analytics).unwrap();
        let decoded: EmailAnalytics = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.template_name, analytics.template_name);
        assert_eq!(decoded.variant_name, analytics.variant_name);
        assert_eq!(decoded.date, analytics.date);
        assert_eq!(decoded.sent_count, analytics.sent_count);
        assert_eq!(decoded.delivered_count, analytics.delivered_count);
        assert_eq!(decoded.opened_count, analytics.opened_count);
        assert_eq!(decoded.clicked_count, analytics.clicked_count);
        assert_eq!(decoded.bounced_count, analytics.bounced_count);
        assert_eq!(decoded.complained_count, analytics.complained_count);
        assert_eq!(decoded.unsubscribed_count, analytics.unsubscribed_count);
    }

    #[test]
    fn email_analytics_serializes_expected_field_names() {
        let value = serde_json::to_value(sample_analytics()).unwrap();
        let obj = value.as_object().unwrap();
        for field in [
            "template_name",
            "variant_name",
            "date",
            "sent_count",
            "delivered_count",
            "opened_count",
            "clicked_count",
            "bounced_count",
            "complained_count",
            "unsubscribed_count",
        ] {
            assert!(obj.contains_key(field), "missing field: {field}");
        }
        assert_eq!(obj.len(), 10);
    }

    #[test]
    fn optional_fields_accept_null() {
        let mut job = serde_json::to_value(sample_job()).unwrap();
        job["started_at"] = json!(null);
        job["completed_at"] = json!(null);
        job["failed_at"] = json!(null);
        job["error_message"] = json!(null);
        let decoded: EmailJob = serde_json::from_value(job).unwrap();
        assert!(decoded.started_at.is_none());
        assert!(decoded.completed_at.is_none());
        assert!(decoded.failed_at.is_none());
        assert!(decoded.error_message.is_none());

        let mut event = serde_json::to_value(sample_event()).unwrap();
        event["email_job_id"] = json!(null);
        event["message_id"] = json!(null);
        let decoded: EmailEvent = serde_json::from_value(event).unwrap();
        assert!(decoded.email_job_id.is_none());
        assert!(decoded.message_id.is_none());

        let mut suppression = serde_json::to_value(sample_suppression()).unwrap();
        suppression["reason"] = json!(null);
        suppression["bounce_type"] = json!(null);
        let decoded: EmailSuppression = serde_json::from_value(suppression).unwrap();
        assert!(decoded.reason.is_none());
        assert!(decoded.bounce_type.is_none());

        let mut analytics = serde_json::to_value(sample_analytics()).unwrap();
        analytics["variant_name"] = json!(null);
        let decoded: EmailAnalytics = serde_json::from_value(analytics).unwrap();
        assert!(decoded.variant_name.is_none());
    }
}
