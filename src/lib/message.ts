export interface Message {
    id: number;
    session_id: number;
    role: string;
    text?: string;
    attachment_path?: string;
}